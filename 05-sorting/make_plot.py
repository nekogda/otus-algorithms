import matplotlib.pyplot as plt
import json
import argparse
import re
from typing import Dict


def _load_data(filename: str) -> Dict:
    with open(filename, 'rb') as fd:
        data = json.load(fd)
    ds = {}
    FNAME = r'(?P<fname>\w+)'                # function name
    SNAME = r'(?:_step_gen_(?P<sname>\w+))'  # name of step generator function
    DSC = r'(?P<dsc>\d+)'                    # numb of elemenets in the dataset
    DST = r'(?P<dst>\w+)'                    # dataset type
    r = re.compile(f'{FNAME}-({SNAME}-)?{DSC}_{DST}-')

    for benchmark in data['benchmarks']:
        m = r.search(benchmark['name'])
        if not m:
            assert False        # regexp should fit to all names
        func_name = f'{m.group("fname")}'
        if m.groupdict()['sname']:
            func_name += f'_{m.group("sname")}'

        dataset_type = m.group('dst')
        if dataset_type not in ds:
            ds[dataset_type] = {}

        if func_name in ds[dataset_type]:
            median = benchmark['stats']['median']
            ds_size = m.group('dsc')
            ds[dataset_type][func_name]['medians'].append(median)
            ds[dataset_type][func_name]['ds_sizes'].append(ds_size)
        else:
            ds[dataset_type][func_name] = {'medians': [], 'ds_sizes': []}

    return ds


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'file_name',
        help='path to the json file with benchmark data',
    )
    parser.add_argument(
        '--ins',
        action='store_true',
        help='Plot insertion_sort algorithm',
    )
    parser.add_argument(
        '--log',
        action='store_true',
        help='Use logarifmic Y-scale',
    )
    return parser.parse_args()


def _make_plot(args: Dict, data: Dict) -> plt:
    cols = 2
    rows, rem = divmod(len(data), cols)
    if rem:
        rows += 1

    fig, axs = plt.subplots(rows, cols)
    for ax, dataset_type in zip(axs.flatten(), data):
        for func_name, measurements in data[dataset_type].items():
            if not args.ins and func_name.startswith('insertion'):
                continue
            ax.plot(
                measurements['ds_sizes'],
                measurements['medians'],
                label=func_name,
            )
        ax.set(
            xlabel='Num of elements in collection',
            ylabel='Time (s)',
            title=f'Sorting algorithms comparsion on {dataset_type}',
        )
        ax.grid()
        if args.log:
            ax.set_yscale('log', basey=10)

    plt.legend()
    plt.subplots_adjust(hspace=0.4)
    return plt


def _main():

    args = _parse_args()
    data = _load_data(args.file_name)
    plt = _make_plot(args, data)
    plt.show()


if __name__ == "__main__":
    _main()
