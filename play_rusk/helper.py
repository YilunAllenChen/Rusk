import pandas as pd
import numpy as np
import random

rd = np.random.random((1000,1000))
df = pd.DataFrame(rd, columns=["column {}".format(i) for i in range(1000)])

df.to_parquet("./random_data.parquet")
