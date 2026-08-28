use crate::Properties;
#[test]
fn from_string() {
    let properties = Properties::from_string("foo=bar\nwill_this_work=I hope so!\nthis_next_line_is_blank!=");

    assert_eq!(properties.get("foo"), String::from("bar"));
    assert_eq!(properties.get("will_this_work"), String::from("I hope so!"));
    assert_eq!(properties.get("this_next_line_is_blank!"), String::from(""))
}

#[test]
fn build() {

    let mut properties : Properties = Default::default();

    assert_eq!(properties.to_string(), "");

    properties.add_set("foo", "bar");
    properties.add_set("hello","world");

    let properties2 = Properties::from_string("foo=bar\nhello=world");

    assert_eq!(properties, properties2);

    let _ = properties.save_to("./test_files/test.properties", Some("This is a test"));

}

#[test]
// file provided by a Neoforge minecraft server
fn read_minecraft_server_properties() {
    let properties = Properties::from_file("./test_files/server.properties").unwrap();

    assert_eq!(properties.get("motd"), "A Minecraft Server");
    assert_eq!(properties.get("level-type"), "minecraft\\:normal");
    assert_eq!(properties.get("gamemode"), "survival");
}

