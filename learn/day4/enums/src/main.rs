#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Hooman {
    name: String,
    gender: Gender,
}

fn main() {
    let aarya = Hooman {
        name: String::from("aarya"),
        gender: Gender::Female,
    };
    let avi = Hooman {
        name: "avi".to_string(),
        gender: Gender::Male,
    };

    println!("{:#?} {:#?}", aarya, avi);

    let avis_gender = gender_using_match(&avi);
    let aaryas_gender = gender_using_if_let(&aarya);

    println!("{} Is avi's gender", avis_gender);
    println!("{} Is aarya's gender", aaryas_gender);

    let person_exist = person_exists(None);

    println!("The person named aarya exists is: {}", person_exist);
}

fn gender_using_match(person: &Hooman) -> String {
    let sex = match person.gender {
        Gender::Male => "Male",
        Gender::Female => "Female",
    };
    sex.to_string()
}

fn gender_using_if_let(person: &Hooman) -> String {
    if let Gender::Male = person.gender {
        String::from("Male")
    } else {
        String::from("Female")
    }
}

fn person_exists(person: Option<&Hooman>) -> bool {
    match person {
        Some(_) => true,
        None => false
    }
}