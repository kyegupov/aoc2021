import itertools

res = 0

digits = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
]

base = "abcdefg"

digits_set = set(digits)

for l in open('input08.txt').readlines():
    (digs, out) = [x.split(' ') for x in l.strip().split(' | ')]
    for p in itertools.permutations(base):
        permuted = set(''.join(sorted(p[ord(c)-ord('a')] for c in w)) for w in digs)
        if permuted == digits_set:
            permuted_out = list(''.join(sorted(p[ord(c)-ord('a')] for c in w)) for w in out)
            x = 0
            for w in permuted_out:
                x *= 10
                x += digits.index(w)
            res += x

print(res)