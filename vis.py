import numpy as np

def main():
    file_name = "mdct_output.csv"
    array = np.genfromtxt(file_name, delimiter=',')
    print(array.shape)

    # plot array[0]
    import matplotlib.pyplot as plt
    plt.plot(np.mean(array, axis=0))
    # plt.plot(array[250])
    plt.show()

if __name__ == "__main__":
    main()