import csv
import decimal
import collections
import statistics

INPUT_PATH = './puzzles/chain/zinc-bench.csv'

buckets: dict[str, list[decimal.Decimal]] = collections.defaultdict(list)

with open(INPUT_PATH, "r", newline="") as input_file:
    reader = csv.reader(input_file)
    for name, time in reader:
        time = decimal.Decimal(time)
        buckets[name].append(time)
        
records = []

for name, times in buckets.items():
    mean = statistics.mean(times)
    stdv = (statistics.stdev(times, mean) / mean) * 100
    mean = mean / 1_000_000
    record = (name, mean, stdv)
    records.append(record)
    
records = sorted(records, key= lambda record: record[1])
    
rows = [('name', 'mean', 'stdv')]
sizes = [4, 4, 4]
        
for name, mean, stdv in records:
    name = '`{}`'.format(' '.join(t.ljust(3) for t in name.split('_')))
    mean = str(round(mean))
    stdv = str(round(stdv, 1))
    row = [name, mean, stdv]
    for i, text in enumerate(row):
        sizes[i] = max(sizes[i], len(text))
    rows.append(row)

for row_index, row in enumerate(rows):
    print('|', end='')
    for text, size in zip(row, sizes):
        text = text.ljust(size)
        print(' {} |'.format(text), end='')
    print()
    if row_index == 0:
        print('|', end='')
        for size in sizes:
            print('{}|'.format('-' * (size + 2)), end='')
        print()