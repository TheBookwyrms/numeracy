# ignore 3, 9, 12d for webwork 2

import numpy as np


class Stats:
    def __init__(self, arr:np.ndarray, sort=False):
        if sort:
            arr.sort()
        self.data = arr

    def mean(self, axis=0):
        return self.data[:, axis].sum() / self.data.shape[0]

    def sample_variance(self):
        return ((self.data-self.mean())**2).sum() / (len(self.data)-1)

    def sample_standard_deviation(self):
        return np.sqrt(self.sample_variance())

    def population_variance(self):
        return ((self.data-self.mean())**2).sum() / len(self.data)

    def population_standard_deviation(self):
        return np.sqrt(self.population_variance())

    def range(self):
        return self.data.max()-self.data.min()
    
    def median(self):
        if len(d)%2 == 1:
            d0 = (len(self.data)-1)/2
            if not d0==int(d0):
                raise Exception()
            return self.data[int(d0)]
        else:
            d0 = (len(self.data))/2
            d1 = (len(self.data))/2-1
            if not ((d0==int(d0)) and (d1==int(d1))):
                raise Exception()
            return (self.data[int(d0)]+self.data[int(d1)])/2
        
    def sum_of_squares_xy(self):
        x = self.data[:, 0]
        y = self.data[:, 1]

        x_mean = self.mean(axis=0)
        y_mean = self.mean(axis=1)

        ssxy = ((x - x_mean)*(y - y_mean)).sum()

        return ssxy
        
    def sum_of_squares_x(self):
        x = self.data[:, 0]

        mean = self.mean(axis=0)

        ssx = ((x - mean)**2).sum()

        return ssx
        
    def sum_of_squares_y(self):
        x = self.data[:, 1]

        mean = self.mean(axis=1)

        ssx = ((x - mean)**2).sum()

        return ssx

        
    def linear_regression(self):
        x = self.data[:, 0]
        y = self.data[:, 1]

        x_mean = self.mean(axis=0)
        y_mean = self.mean(axis=1)

        ssxy = self.sum_of_squares_xy()
        ssx = self.sum_of_squares_x()

        b1 = ssxy/ssx
        b0 = y_mean - b1*x_mean

        #print(b0, b1)
        return lambda var : b0 + b1*var
    
    def correlation_coefficient(self):
        return self.sum_of_squares_xy()/np.sqrt(self.sum_of_squares_x()*self.sum_of_squares_y())
    
    def coefficient_of_determination(self):
        return self.correlation_coefficient()**2
    
def factorial(n):
    p = 1
    for i in range(1, n+1):
        p*= i
    return p

def choose(n, k):
    return factorial(n)/(factorial(k)*factorial(n-k))

print(choose(9, 2) * choose(19, 7))
print(choose(19+9, 9) - choose(19, 9))
print(choose(13, 2) * choose(13, 2) * choose(26, 3) * 1)
print(choose(21, 3), choose(14, 3))



d = np.array([854, 884, 904, 921, 869, 888, 847, 911, 856, 934, 861, 897, 929, 911, 830])
d2 = np.array([
    [38, 44],
    [41, 53],
    [34, 44],
    [45, 54],
    [71, 84],
    [56, 68],
    [31, 36],
    [33, 40],
    [67, 79],
    [66, 79],
])
dataset = Stats(d, sort=True)
dataset = Stats(d2)

#print(dataset.sum_of_squares_xy(), dataset.sum_of_squares_x(), dataset.sum_of_squares_y())
#print(dataset.linear_regression())
#print(dataset.correlation_coefficient())
#print(dataset.coefficient_of_determination())

#print(dataset.range())
#print(dataset.mean())
#print(dataset.sample_variance())
#print(dataset.median())
#print(dataset.sample_standard_deviation())