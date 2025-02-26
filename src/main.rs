
fn main (){
    let sized: MySuperSliceable<[u8; 8]> = MySuperSliceable {
        info: 17,
        data:[0;8],
    };

    let dynamic = &sized;

    //prints: "17 [0, 0, 0, 0, 0, 0, 0, 0]"
    println!("{} {:?}", dynamic.info, dynamic.data);
}

struct MySuperSliceable<T: ?Sized> {
    info:u32,
    data: T,
}