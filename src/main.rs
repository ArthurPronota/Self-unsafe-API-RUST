use std::slice ;

fn split_at_mute<T>(slice_in: &mut[T], mid: usize) ->(&mut [T], &mut [T]) {

    let len_slice = slice_in.len() ;

    assert!(mid <= len_slice, "mid: {mid} > len_slice: {len_slice}") ;

    let ptr = slice_in.as_mut_ptr() ;

    // SAFETY: мы проверили, что `mid <= len`.
    // - ptr.add(mid) остаётся в пределах аллокации (или на её конце)
    // - два среза не пересекаются: [0, mid) и [mid, len)
    // - оба указателя валидны для T, alignment сохраняется
    unsafe {
        (
            // unsafe-функция, которая создаёт изменяемый срез &mut [T] из сырого указателя и длины. 
            slice::from_raw_parts_mut(
                ptr,   // сырой указатель на первый элемент
                mid     // Количество элементов в срезе
            ),
            // unsafe-функция, которая создаёт изменяемый срез &mut [T] из сырого указателя и длины. 
            std::slice::from_raw_parts_mut(
                ptr
                        .add(   // сдвиг сырого указателя вперёд на mid элементов типа T.
                            mid
                        ), 
                len_slice - mid // Количество элементов в срезе
            )
        )
    } 

}

fn main() {
    let mut v = vec![1,2,3,4] ;

    let (v_first, v_second) = split_at_mute(&mut v, 2) ;

    println!("Before: v_first: {v_first:?}, v_second: {v_second:?}") ;  // Out: Before: v_first: [1, 2], v_second: [3, 4]

    v_first[0] *= 10 ;
    v_second[1] *= 10 ;

    println!("After: v_first: {v_first:?}, v_second: {v_second:?}") ;   // Out: After: v_first: [10, 2], v_second: [3, 40]
}
