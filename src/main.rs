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

        "object" => {
            Command::new("mkdir").arg(format!("objects/{}", &args[2])).output().expect("Failed to create object directory");
            let mut file = std::fs::File::create(format!("objects/{a}/{a}.yy", a=&args[2])).expect("Failed to create YY file");
            file.write_all(format!("{{\
              \"spriteId\": null,\
              \"solid\": false,\
              \"visible\": true,\
              \"managed\": true,\
              \"spriteMaskId\": null,\
              \"persistent\": true,\
              \"parentObjectId\": null,\
              \"physicsObject\": false,\
              \"physicsSensor\": false,\
              \"physicsShape\": 1,\
              \"physicsGroup\": 1,\
              \"physicsDensity\": 0.5,\
              \"physicsRestitution\": 0.1,\
              \"physicsLinearDamping\": 0.1,\
              \"physicsAngularDamping\": 0.1,\
              \"physicsFriction\": 0.2,\
              \"physicsStartAwake\": true,\
              \"physicsKinematic\": false,\
              \"physicsShapePoints\": [],\
              \"eventList\": [\
                {{\"isDnD\":false,\
                \"eventNum\":0,\
                \"eventType\":0,\
                \"collisionObjectId\":null,\
                \"resourceVersion\":\"1.0\",\
                \"name\":\"\",\
                \"tags\":[],\
                \"resourceType\":\"GMEvent\",\
                }},\
              ],\
              \"properties\": [],\
              \"overriddenProperties\": [],\
              \"parent\": {{\
                \"name\": \"Objects\",\
                \"path\": \"folders/Objects.yy\",\
              }},\
              \"resourceVersion\": \"1.0\",\
              \"name\": \"{}\",\
              \"tags\": [],\
              \"resourceType\": \"GMObject\",\
            }}", &args[2]).as_bytes()).expect("Failed to write to YY file");
            Command::new("touch").arg(format!("objects/{}/Create_0.gml", &args[2])).output().expect("Failed to create GML file");
        },

        _ => panic!("Not a valid/supported asset type: {}", &args[1])
    }
}
