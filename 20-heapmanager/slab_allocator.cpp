#include <cassert>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
using namespace std::chrono;

#define MIN_SLAB_ORDER 12
#define MIN_SLAB_OBJECTS 64
#define MAX_OBJECT_SIZE 1048576 /* 1 MB */
#define MIN_OBJECT_SIZE sizeof(size_t)

#define BENCH_PERIOD 10000
#define BENCH_ROUNDS 10000000
#define BENCH_BLOCK_SIZE 1024

enum cache_list_t {
  L_FREE = 0,
  L_PART = 1,
  L_FULL = 2,
};

typedef struct object_node {
  object_node *next_node = NULL;
} object_node_t;

struct slab {
  slab *next_slab = NULL;
  slab *prev_slab = NULL;
  size_t free_objects = 0;
  object_node *next_free_object = NULL;
};

struct cache {
  slab *lists[3] = {NULL, NULL, NULL};
  size_t object_size;  /* size of object in the cache*/
  uint slab_order;     /* slab size */
  size_t slab_objects; /* number of objects in the single slab */
};

void *alloc_slab(int order);
void free_slab(void *slab);

void cache_setup(struct cache *cache, size_t object_size);
void slab_setup(cache *cache);
void cache_release(struct cache *cache);

void push_free_obj_to_slab(void *obj, slab *s);
void *pop_free_obj_from_slab(slab *s);

void push_slab_to_list(slab *s, cache *c, cache_list_t lst);
slab *pop_slab_from_list(cache *c, cache_list_t lst, slab *s);

void *cache_alloc(struct cache *cache);
void cache_free(struct cache *cache, void *ptr);
void cache_shrink(struct cache *cache);

void *alloc_slab(int order) {
  return aligned_alloc((size_t)pow(2, (MIN_SLAB_ORDER + order)),
                       (size_t)pow(2, (MIN_SLAB_ORDER + order)));
}

void free_slab(void *slab) { free(slab); }

// initialize cache
void cache_setup(struct cache *cache, size_t object_size) {
  assert(object_size < MAX_OBJECT_SIZE);
  assert(object_size >= MIN_OBJECT_SIZE);
  int order = 0;
  while ((size_t)pow(2, (MIN_SLAB_ORDER + order)) <=
             object_size * MIN_SLAB_OBJECTS &&
         order < 10) {
    ++order;
  }
  cache->object_size = object_size;
  cache->slab_order = order;
  cache->slab_objects =
      ((size_t)(pow(2, (MIN_SLAB_ORDER + order)) - sizeof(slab)) / object_size);
  // cache configured and now we must initialize slab
  slab_setup(cache);
}

void slab_setup(cache *cache) {
  void *buf = alloc_slab(cache->slab_order);
  if (buf == NULL) {
    std::cerr << "can't allocate memory. Aborting." << std::endl;
    abort();
  }
  ((object_node *)buf)->next_node = NULL;

  slab *s = (slab *)((char *)buf +
                     (size_t)pow(2, MIN_SLAB_ORDER + cache->slab_order) -
                     sizeof(slab));
  s->free_objects = cache->slab_objects;
  s->next_slab = NULL;
  s->prev_slab = NULL;

  for (size_t i = 1; i < s->free_objects; ++i) {
    object_node *curr_obj_ptr =
        (object_node *)((char *)buf + cache->object_size * i);
    curr_obj_ptr->next_node =
        (object_node *)((char *)buf + cache->object_size * (i - 1));
  }

  s->next_free_object =
      (object_node *)((char *)buf + cache->object_size * (s->free_objects - 1));

  push_slab_to_list(s, cache, L_FREE);
}

void *get_slab_begin(cache *c, void *obj) {
  size_t slab_size = (size_t)pow(2, (MIN_SLAB_ORDER + c->slab_order)) - 1;
  void *slab_begin_addr = (void *)((size_t)obj & (~slab_size));
  return (void *)slab_begin_addr;
}

void cache_release(struct cache *cache) {
  slab *s;
  for (int l_type = L_FREE; l_type <= L_FULL; ++l_type) {
    while (cache->lists[l_type]) {
      s = pop_slab_from_list(cache, (cache_list_t)l_type, cache->lists[l_type]);
      free_slab(get_slab_begin(cache, s));
    }
  }
}

void *pop_free_obj_from_slab(slab *s) {
  object_node *tmp = s->next_free_object->next_node;
  object_node *result = s->next_free_object;
  s->next_free_object = tmp;
  s->free_objects--;
  return (void *)result;
}

void push_free_obj_to_slab(void *obj, slab *s) {
  ((object_node *)obj)->next_node = s->next_free_object;
  s->next_free_object = (object_node *)obj;
  s->free_objects++;
}

void push_slab_to_list(slab *s, cache *c, cache_list_t lst) {
  s->next_slab = NULL;
  s->prev_slab = NULL;
  if (c->lists[lst] != NULL) {
    s->next_slab = c->lists[lst];
    c->lists[lst]->prev_slab = s;
    c->lists[lst] = s;
  } else {
    c->lists[lst] = s;
  }
}

slab *pop_slab_from_list(cache *c, cache_list_t lst, slab *s) {
  slab *s_next = s->next_slab;
  slab *s_prev = s->prev_slab;
  if (s_prev) {
    s_prev->next_slab = s_next;
  } else {
    c->lists[lst] = s_next;
  }
  if (s_next) {
    s_next->prev_slab = s_prev;
  }
  return s;
}

void *cache_alloc(struct cache *cache) {
  void *obj = NULL;
  if (cache->lists[L_PART]) {
    obj = pop_free_obj_from_slab(cache->lists[L_PART]);
    if (cache->lists[L_PART]->free_objects == 0) {
      slab *s = pop_slab_from_list(cache, L_PART, cache->lists[L_PART]);
      push_slab_to_list(s, cache, L_FULL);
    }
  } else {
    if (!cache->lists[L_FREE]) {
      slab_setup(cache);
    }
    obj = pop_free_obj_from_slab(cache->lists[L_FREE]);
    slab *s = pop_slab_from_list(cache, L_FREE, cache->lists[L_FREE]);
    if (s->free_objects == 0) {
      push_slab_to_list(s, cache, L_FULL);
    } else {
      push_slab_to_list(s, cache, L_PART);
    }
  }
  return obj;
}

void cache_free(struct cache *cache, void *obj) {
  void *slab_begin_addr = get_slab_begin(cache, obj);
  slab *s = (slab *)((char *)slab_begin_addr +
                     (size_t)pow(2, MIN_SLAB_ORDER + cache->slab_order) -
                     sizeof(slab));
  cache_list_t current_list = s->free_objects == 0 ? L_FULL : L_PART;
  push_free_obj_to_slab(obj, s);
  if (s->free_objects == cache->slab_objects) {
    slab *tmp = pop_slab_from_list(cache, current_list, s);
    push_slab_to_list(tmp, cache, L_FREE);
  } else {
    if (current_list != L_PART) {
      slab *tmp = pop_slab_from_list(cache, current_list, s);
      push_slab_to_list(tmp, cache, L_PART);
    }
  }
}

void cache_shrink(struct cache *cache) {
  while (cache->lists[L_FREE]) {
    slab *s = pop_slab_from_list(cache, L_FREE, cache->lists[L_FREE]);
    free_slab(get_slab_begin(cache, s));
  }
}

void print_report(const char *name, size_t block_size,
                  duration<double> elapsed) {
  std::cout.precision(6);
  std::cout << std::fixed;
  std::cout << "allocator: " << std::setw(6) << name << ", ";
  std::cout << "blk_size: " << std::setw(6) << block_size << ", ";
  std::cout << "iterations: " << BENCH_ROUNDS << ", ";
  std::cout << "elapsed (s): " << elapsed.count() << ", ";
  std::cout << "ns/iter: " << elapsed.count() / BENCH_ROUNDS * 1000000000;
  std::cout << std::endl;
}

void malloc_bench(size_t block_size) {
  auto start = system_clock::now();
  void *tmp[BENCH_PERIOD];
  int i = 0;
  while (i < BENCH_ROUNDS) {
    tmp[i % BENCH_PERIOD] = malloc(block_size);
    *(char *)tmp[i % BENCH_PERIOD] = 'x';
    ++i;
    if (i % BENCH_PERIOD == 0) {
      for (int j = 0; j < BENCH_PERIOD; ++j) {
        free(tmp[j]);
      }
      i += BENCH_PERIOD;
    }
  }
  auto end = system_clock::now();
  duration<double> elapsed_seconds = end - start;
  print_report("malloc", block_size, elapsed_seconds);
}

void slab_bench(size_t block_size) {
  auto start = system_clock::now();
  struct cache my_cache;
  cache_setup(&my_cache, block_size);
  void *tmp[BENCH_PERIOD];
  int i = 0;
  while (i < BENCH_ROUNDS) {
    tmp[i % BENCH_PERIOD] = cache_alloc(&my_cache);
    *(char *)tmp[i % BENCH_PERIOD] = 'x';
    ++i;
    if (i % BENCH_PERIOD == 0) {
      for (int j = 0; j < BENCH_PERIOD; ++j) {
        cache_free(&my_cache, tmp[j]);
      }
      i += BENCH_PERIOD;
    }
  }
  cache_release(&my_cache);
  auto end = system_clock::now();
  duration<double> elapsed_seconds = end - start;
  print_report("slab", block_size, elapsed_seconds);
}

int main(void) {
  // simple unit-tests
  {
    struct cache my_cache;
    cache_setup(&my_cache, 64);
    assert(my_cache.lists[L_FREE] != NULL);
    assert(my_cache.lists[L_PART] == NULL);
    assert(my_cache.lists[L_FULL] == NULL);
    assert(my_cache.lists[L_FREE]->free_objects == 127);
    assert(my_cache.slab_objects == 127);
    cache_release(&my_cache);
  }

  {
    struct cache my_cache;
    cache_setup(&my_cache, 64);
    cache_shrink(&my_cache);
    assert(my_cache.lists[L_FREE] == NULL);
    cache_release(&my_cache);
  }

  {
    struct cache my_cache;
    cache_setup(&my_cache, 64);
    cache_shrink(&my_cache);
    assert(my_cache.lists[L_FREE] == NULL);
    void *ptr = cache_alloc(&my_cache);
    assert(my_cache.lists[L_FREE] == NULL);
    assert(my_cache.lists[L_PART] != NULL);
    assert(my_cache.lists[L_PART]->free_objects == 126);
    cache_free(&my_cache, ptr);
    assert(my_cache.lists[L_FREE] != NULL);
    assert(my_cache.lists[L_PART] == NULL);
    assert(my_cache.lists[L_FREE]->free_objects == 127);
    cache_release(&my_cache);
  }

  {
    struct cache my_cache;
    cache_setup(&my_cache, 64);
    cache_shrink(&my_cache);
    void *tmp[254];
    for (int i = 0; i < 254; ++i) {
      tmp[i] = cache_alloc(&my_cache);
    }
    assert(my_cache.lists[L_FREE] == NULL);
    assert(my_cache.lists[L_PART] == NULL);
    assert(my_cache.lists[L_FULL] != NULL);
    assert(my_cache.lists[L_FULL]->next_slab != NULL);
    assert(my_cache.lists[L_FULL]->free_objects == 0);
    assert(my_cache.lists[L_FULL]->next_slab->free_objects == 0);
    cache_free(&my_cache, tmp[0]);
    assert(my_cache.lists[L_FULL]->next_slab == NULL);
    assert(my_cache.lists[L_FULL]->free_objects == 0);
    assert(my_cache.lists[L_PART]->next_slab == NULL);
    assert(my_cache.lists[L_PART]->free_objects == 1);
    for (int i = 1; i < 254; ++i) {
      cache_free(&my_cache, tmp[i]);
    }
    cache_release(&my_cache);
  }

  // benchmarks
  for (int i = 8; i <= 8192; i <<= 1) {
    malloc_bench(i);
    slab_bench(i);
  }

  return 0;
}
