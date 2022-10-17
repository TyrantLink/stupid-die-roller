from subprocess import Popen,PIPE
from os.path import exists


FILE_PATH = f'{"../" if exists("main.rs") else ""}target/release/stupid-die-roller'

if not exists(FILE_PATH):
	print('file not found, please run `cargo build --release`')


def benchmark(high_acc:bool):
	proc = Popen([FILE_PATH,f'--{"high-acc-" if high_acc else ""}benchmark'],stdout=PIPE,stdin=PIPE)
	proc.stdin.write(b'20\n')
	proc.stdin.flush()
	while True:
		line = proc.stdout.readline().decode()
		if line.startswith('benchmark complete:'):
			result = float(line.split(' ')[2])
			print(f'{"{:,}".format(round(result,2))} rolls/s')
			return result

def main():
	high_acc = input('run high accuracy benchmark? [y/N] ').lower() in ['y','yes','true']
	loops = int(input('how many times would you like to run the benchmark? [5]') or 5)
	benchmarks = [benchmark(high_acc) for i in range(loops)]
	print(f'{"[HIGH-ACC] " if high_acc else ""}average of {loops} benchmarks: {"{:,}".format(round(sum(benchmarks)/loops,2))} rolls/s')
	
if __name__ == '__main__':
	main()