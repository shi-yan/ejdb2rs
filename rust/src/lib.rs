
extern crate ejdb_sys;
use ejdb_sys::ejdb_init;

fn test() {

    let a = unsafe {
        let rc = ejdb_init();
        print!("{:?}", rc);
        rc
    };

    print!("{:?}", a);
    
}
