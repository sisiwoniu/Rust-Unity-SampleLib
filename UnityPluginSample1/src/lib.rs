// #[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn add_ver_2(src: *mut i32) -> *const i32 {
    Box::into_raw(Box::new(*src + 1))
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn test_fn_1(srcPtr: *mut i32, length: i32) {
    //lengthは要素数
    let data = std::slice::from_raw_parts_mut(srcPtr, (length) as usize);

    for i in 0..length {
        let index = i as usize;
        data[index] *= 2;
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct TestStruct {
    pub X: i32,
}

#[no_mangle]
pub unsafe extern "C" fn test_fn_2(src_ptr: *mut TestStruct) -> *mut TestStruct {
    let mut data = Box::from_raw(src_ptr);
    let mut value = 10000.0_f64;
    for _i in 0..4 {
        value = value.sqrt();
    }
    data.X = value.floor() as i32;
    Box::into_raw(data)
}

#[no_mangle]
pub extern "C" fn test_fn_3(data: &mut TestStruct) {
    let mut value = 10000.0_f64;
    for _i in 0..4 {
        value = value.sqrt();
    }
    data.X = value.floor() as i32;
}

#[no_mangle]
pub extern "C" fn test_fn_4(data: TestStruct) -> TestStruct {
    let mut value = 10000.0_f64;
    for _i in 0..4 {
        value = value.sqrt();
    }
    let mut data2 = data;
    data2.X = value.floor() as i32;
    data2
}

#[no_mangle]
pub unsafe extern "C" fn test_fn_5(src_ptr: *mut TestStruct, length: i32) {
    let data = std::slice::from_raw_parts_mut(src_ptr, length as usize);

    for i in 0..length {
        let mut value = 10000.0_f64;
        for _y in 0..4 {
            value = value.sqrt();
        }
        let index = i as usize;
        data[index].X = value.floor() as i32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn delete_ptr(src: *mut i32) {
    Box::from_raw(src);
}
