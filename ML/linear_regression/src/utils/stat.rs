pub fn mean(values: &Vec<f32>) -> f32 {
    if values.len() == 0 {
        return 0f32;
    }

    return values.iter().sum::<f32>() / (values.len() as f32);
}

pub fn variance(values: &Vec<f32>) -> f32 {
    if values.len() == 0 {
        return 0f32;
    }

    let mean = mean(values);
    return values.iter()
            .map(|x| f32::powf(x - mean, 2 as f32))
            .sum::<f32>() / values.len() as f32;
}

pub fn covariance(x_values: &Vec<f32>, y_values: &Vec<f32>) -> f32 {
    if x_values.len() != y_values.len() {
        panic!("x_values and y_values must be of equal length.")
    }
    
    let length: usize = x_values.len();

    if length == 0usize {
        return 0f32;
    }

    let mut covariance: f32 = 0f32;
    let mean_x = mean(x_values);
    let mean_y = mean(y_values);

    for i in 0..length {
        covariance += (x_values[i] - mean_x) * (y_values[i] - mean_y)
    }

    return covariance / length as f32;
}
