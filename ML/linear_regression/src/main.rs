use ml_models::regression::linear_regression;
use ml_models::gradient_descent::gradient_descent;

pub fn main() {
    let mut model = linear_regression::LinearRegression::new();
    let x_values = vec![1f32, 2f32, 3f32, 4f32, 5f32];
    let y_values = vec![1f32, 2f32, 3f32, 4f32, 5f32];

    let tuple = gradient_descent::linear_regression(&x_values, &y_values, 10000, 0.005);

    let intercept = tuple.0;
    let coefficient = tuple.1;

    model.fit(&x_values, &y_values);

    let coefficient_1 = model.coefficient;
    let intercept_1 = model.intercept;

    println!("Coefficient: {0}", model.coefficient.unwrap());
    println!("Intercept: {0}", model.intercept.unwrap());
    println!("Accuracy: {0}", model.evaluate(&x_values, &y_values));

    let y_prediction: Vec<f32> = model.predict_list(&x_values);
    let y_prediction_f64: Vec<f64> = y_prediction
        .into_iter()
        .map(|x| x as f64)
        .collect();

    let x_values_f64: Vec<f64> = x_values.into_iter().map(|x| x as f64).collect();
    let y_values_f64: Vec<f64> = y_values.into_iter().map(|x| x as f64).collect();

    let mut actual: Vec<(f64, f64)> = Vec::new();
    let mut prediction: Vec<(f64, f64)> = Vec::new();

    for i in 0..x_values_f64.len() {
        actual.push((x_values_f64[i], y_values_f64[i]));
        prediction.push((x_values_f64[i], y_prediction_f64[i]));
    }
}
