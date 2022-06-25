use std::env;
use std::process::Command;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Wrong number of arguments");

    match &args[1][..] {
        "script" => {
            Command::new("mkdir").arg(format!("scripts/{}", &args[2])).output().expect("Failed to create script directory");
            let mut file = std::fs::File::create(format!("scripts/{a}/{a}.yy", a=&args[2])).expect("Failed to create YY file");
            file.write_all(format!(
                "{{\
                \"isDnD\":false,\
                \"isCompatibility\":false,\
                \"parent\":{{\
                \"name\":\"Scripts\",\
                \"path\":\"folders/Scripts.yy\",\
                }},\
                \"resourceVersion\":\"1.0\",\
                \"name\":\"{}\",\
                \"tags\":[],\
                \"resourceType\":\"GMScript\",\
                }}", &args[2]).as_bytes()).expect("Failed to write to YY file");
            Command::new("touch").arg(format!("scripts/{a}/{a}.gml", a=&args[2])).output().expect("Failed to create GML file");
        },

        "object" => todo!(),

        _ => panic!("Not a valid/supported asset type: {}", &args[1])
    }
}
