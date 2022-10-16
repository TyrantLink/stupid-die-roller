from random import randint
from time import perf_counter

highest = 0
highest2 = 0
inp = int(input('d'))
st = perf_counter()
iters = 0

while highest != inp:
  iters += 1
  if randint(1,inp) == highest+1:
    highest += 1
    if highest > highest2:
      highest2 = highest
      print(f'{highest2}/{inp} in {perf_counter()-st} seconds and {iters} iters | {iters/(perf_counter()-st)} it/s')
  else: highest = 0
  # if highest == inp: break

print(f'total time: {perf_counter()-st}')
