import csv
import matplotlib.pyplot as plt
import numpy as np
from sklearn.linear_model import LinearRegression
from sklearn.preprocessing import PolynomialFeatures

def fit_polynomial(x, y, degree=3):
    poly_features = PolynomialFeatures(degree=degree)
    x_poly = poly_features.fit_transform(x.reshape(-1, 1))
    model = LinearRegression()
    model.fit(x_poly, y)
    return model, poly_features

param = []
with open("LevelCost.csv") as f:
    for row in csv.reader(f):
        param.append(row)

level = [[int(param[x][2]),int(param[x][4])] for x in range(1,len(param)-1)]
level = level[1:]
print(*level,sep="\n")

segments = [0,199,249,len(level)]

x = np.array([x[0] for x in level])
y = np.array([x[1] for x in level])

models = []
for i in range(len(segments) - 1):
    start, end = segments[i], segments[i + 1]
    x_segment = x[start:end]
    y_segment = y[start:end]
    
    model, poly_features = fit_polynomial(x_segment, y_segment, degree=3)
    models.append((model, poly_features))
    
def predict(model, poly_features, x):
    x_poly = poly_features.transform(x.reshape(-1, 1))
    return model.predict(x_poly)
    
predictions = []
for i in range(len(segments) - 1):
    start, end = segments[i], segments[i + 1]
    x_segment = x[start:end]
    
    model, poly_features = models[i]
    y_pred = predict(model, poly_features, x_segment)
    
    predictions.extend(y_pred)
    
plt.plot(x,y)
plt.plot(x,predictions)
plt.show()

for model,poly_features in models:
    coefficients = model.coef_
    intercept = model.intercept_

    degree = poly_features.degree

    print("Coefficients:", coefficients)
    print("Intercept:", intercept)
    print("Degree:", degree)