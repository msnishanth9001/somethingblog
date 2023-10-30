import random

class bubbleSort:
	def __init__(self):
		self.random_numbers = [random.randint(1, 10000) for _ in range(1000)]
		self.iterations = 0

	def sort(self):
		num_list = self.random_numbers
		for i in range(len(num_list)):
			# print(i, num_list)
			for j in range(0, len(num_list)-i-1):
				self.iterations += 1
				if num_list[j] > num_list[j+1]:
					num_list[j], num_list[j+1] = num_list[j+1], num_list[j]
		return num_list

	def validate(self, num_list):
		return num_list == sorted(self.random_numbers)

search = bubbleSort()
print(search.validate(search.sort()))