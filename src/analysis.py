import numpy as np
import powerlaw
import sys, os
import matplotlib.pyplot as plt

# CONVERTS FILE DATA INTO AN ARRAY
def read_numbers_from_file(path):
    data = [] 
    with open(path, 'r') as file:  
        # ITERATIVELY READS LINE, ADDS NUMBER TO `data`
        for line in file:
            data.append(int(line.strip()))
    return data

# ACCEPTS LIST OF NUMERICAL VALUES
def print_statistics(distribution):
    distribution = np.array(distribution)
    print("Mean: {}".format(np.mean(distribution)))
    print("Median: {}".format(np.median(distribution)))
    print("Standard Deviation: {}".format(np.std(distribution)))
    print("y-max: ({}, {}), x-max: ({}, {})".format(np.argmax(distribution), np.max(distribution), len(distribution) - 1, distribution[-1]))
    print("y-min: ({}, {}), x-min: ({}, {})".format(np.argmin(distribution), np.min(distribution), 0, distribution[0]))
                           
# ACCEPTS LIST OF NUMERICAL VALUES, PERFORMS KOLMOGROV-SMIRNOV DISTANCE TEST
def powerlaw_analysis(path):
    distribution = read_numbers_from_file(path)                                     # READS VALUES FROM DATA IN `path`
    os.remove(path)                                                                 # DELETES FILE
    distribution = np.array(distribution)                                                                       
    data = distribution[distribution > 0]                                           # POWERLAW REJECTS NON-POSITIVE VALUES FILTERS 0 OR NEGATIVE VALUES
    sys.stdout = open(os.devnull, 'w')                                              # BLOCKS CALLS TO PRINT
    
    # FOR SOME REASON, THIS LINE OUTPUTS MANY VALUES 
    results = powerlaw.Fit(data)                                                    # FITS DATA TO POWERLAW
    sys.stdout = sys.__stdout__                                                     # RE-OPENS CALLS TO PRINT

    ks_distance = results.power_law.KS()
    print("KS Distance: {}".format(ks_distance))
    print_statistics(distribution)
    scatter_plot(distribution, path)

# PLOTS SCATTERPLOT
def scatter_plot(distribution, path):
    x_values = list(range(len(distribution)))
    plt.scatter(x_values, distribution, s=10)
    plt.title('Scatter Plot of {}'.format(path))
    plt.xlabel('Number of Neighbors (#)')
    plt.ylabel('Frequency (#)')
    plt.grid(True) 
    plt.show()

# MATCHES COMMAND LINE ARGUMENT TO CORRECT FUNCTION AND ARGUMENT
if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python {} <path_to_file>".format(sys.argv[0]))
        sys.exit(1)
    path = sys.argv[1]
    powerlaw_analysis(path)

