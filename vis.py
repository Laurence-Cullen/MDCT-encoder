import numpy as np


def main():
    file_name = "mdct_output.csv"
    array = np.genfromtxt(file_name, delimiter=',')
    print(array.shape)

    # plot array[0]
    import matplotlib.pyplot as plt
    # plt.plot(np.mean(array, axis=0))
    # plt.plot(array[250])

    # normalise array
    # array = array - np.min(array)
    # array = array / np.max(array)

    # array = np.log(array + 1)

    # plot heat map of 2d array
    plt.imshow(array[:, 20:100], cmap='hot', interpolation='nearest')
    plt.show()


if __name__ == "__main__":
    main()
