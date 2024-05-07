// 实现一个冒泡排序
pub fn _bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                // swap 替换索引位置的值
                arr.swap(j, j + 1);
            }
        }
    }
}

// 实现一个快速排序
pub fn _quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let pivot = arr[0];
    let mut i = 0;
    let mut j = len - 1;
    while i < j {
        while i < j && arr[j] >= pivot {
            j -= 1;
        }
        arr[i] = arr[j];
        while i < j && arr[i] <= pivot {
            i += 1;
        }
        arr[j] = arr[i];
    }
    arr[i] = pivot;
    _quick_sort(&mut arr[0..i]);
    _quick_sort(&mut arr[i + 1..len]);
}
