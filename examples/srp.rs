#[derive(serde::Serialize)]
enum Gender {
    Male,
    #[allow(dead_code)]
    Female,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Employee {
    family_name: String,
    given_name: String,
    age: u32,
    gender: Gender,
}

impl Employee {
    fn new(family_name: &str, given_name: &str, age: u32, gender: Gender) -> Self {
        Self {
            family_name: family_name.to_owned(),
            given_name: given_name.to_owned(),
            age,
            gender,
        }
    }
}

/// あまり良くない実装
///
/// JSON、XML及びYAMLを求めるグループが異なる場合、あまり良くない実装と判断される。
/// 関数が分離しているから、問題ないという考え方もあるかもしれない。
/// ただし、モジュールに分かれていないため、再コンパイルが必要になる。
struct EmployeeExpresser;

impl EmployeeExpresser {
    fn to_json(employee: &Employee) -> String {
        serde_json::to_string(employee).unwrap()
    }

    fn to_xml(employee: &Employee) -> String {
        serde_xml_rs::to_string(employee).unwrap()
    }

    fn to_yaml(employee: &Employee) -> String {
        serde_yaml::to_string(employee).unwrap()
    }
}

/// 上記よりも、おそらく良い実装
///
/// JSON、XML及びYAMLを求めるグループが異なる場合、あまり良くない実装と判断される。
/// この例では同じモジュールに存在するため、あるグループで求められる実装が
/// 変更された場合、再コンパイルが必要になる。
/// 再コンパイルが望ましくないのであれば、別モジュールに分けて実装する。
struct JsonEmployeeExpresser;

impl JsonEmployeeExpresser {
    fn to_json(employee: &Employee) -> String {
        serde_json::to_string(employee).unwrap()
    }
}

struct XmlEmployeeExpresser;

impl XmlEmployeeExpresser {
    fn to_xml(employee: &Employee) -> String {
        serde_xml_rs::to_string(employee).unwrap()
    }
}

struct YamlEmployeeExpresser;

impl YamlEmployeeExpresser {
    fn to_yaml(employee: &Employee) -> String {
        serde_yaml::to_string(employee).unwrap()
    }
}

fn main() {
    let employee = Employee::new("Tokugawa", "Ieyasu", 46, Gender::Male);
    println!("{}", EmployeeExpresser::to_json(&employee));
    println!("{}", EmployeeExpresser::to_xml(&employee));
    println!("{}", EmployeeExpresser::to_yaml(&employee));

    println!("{}", JsonEmployeeExpresser::to_json(&employee));
    println!("{}", XmlEmployeeExpresser::to_xml(&employee));
    println!("{}", YamlEmployeeExpresser::to_yaml(&employee));
}
