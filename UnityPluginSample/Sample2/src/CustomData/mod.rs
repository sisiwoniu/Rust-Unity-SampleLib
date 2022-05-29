use std::sync::{Arc, Mutex};

use crate::ThreadPool::ThreadPool;

use self::matrix::Quaternion;

pub mod matrix;
pub mod vec3;

#[no_mangle]
pub unsafe extern "C" fn add_value_vec3_array(data: &mut vec3::Vec3, len: usize, add_value: f32) {
    let arr = std::slice::from_raw_parts_mut(data, len);

    for d in arr {
        *d += add_value;
    }
}

#[no_mangle]
pub unsafe extern "C" fn add_vec_vec3_array(
    data: &mut vec3::Vec3,
    len: usize,
    add_value: vec3::Vec3,
) {
    let arr = std::slice::from_raw_parts_mut(data, len);

    for d in arr {
        *d += add_value;
    }
}

#[no_mangle]
pub unsafe extern "C" fn vec3_array_lerp(
    data: &mut vec3::Vec3,
    start_data: &mut vec3::Vec3,
    end_data: &mut vec3::Vec3,
    len: usize,
    t: f32,
) {
    let arr = std::slice::from_raw_parts_mut(data, len);
    let start_arr = std::slice::from_raw_parts_mut(start_data, len);
    let end_arr = std::slice::from_raw_parts_mut(end_data, len);

    for i in 0..len {
        let end = end_arr[i];
        let start = start_arr[i];
        arr[i] = vec3::Vec3::lerp(&start, &end, t);
    }
}

#[no_mangle]
pub extern "C" fn rotateY(angle: f32) -> matrix::Quaternion {
    matrix::Quaternion::rotateY(angle)
}

#[no_mangle]
pub unsafe extern "C" fn rotate_y_array(data: &mut Quaternion, angle_data: &f32, len: usize) {
    let data_arr = std::slice::from_raw_parts_mut(data, len);

    let angle_arr = std::slice::from_raw_parts(angle_data, len);

    let data_arr_arc = Arc::new(Mutex::new(data_arr));

    let angle_arr_arc = Arc::new(Mutex::new(angle_arr));

    {
        //ThreadPoolのドロップタイミングで実行中の前タスクの終了待ちをやるので、独自のスコープを用意している
        let thread_pool = ThreadPool::new(6);

        for i in 0..len {
            let data_clone = Arc::clone(&data_arr_arc);
            let angle_clone = Arc::clone(&angle_arr_arc);
            thread_pool.execute(move || {
                let result = rotateY(angle_clone.lock().unwrap()[i]);
                data_clone.lock().unwrap()[i] = result;
                println!("result == {:?}; index == {}", result, i);
            });
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::matrix::Quaternion;
    use super::rotateY;

    #[test]
    fn test_rotateY() {
        let tmp = rotateY(50.0);

        println!("quaternion == {:?}", tmp);
    }

    #[test]
    fn test_thread_pool() {
        let mut q_array = [Quaternion::new(); 10000];
        let mut angle_array = [1_f32; 10000];

        unsafe {
            super::rotate_y_array(&mut q_array[0], &mut angle_array[0], 10000);
        }

        print!("test thread pool!")
    }
}
