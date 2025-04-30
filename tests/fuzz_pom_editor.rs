use maven_rs::{
    pom::{Dependency, Repository, editor::PomEditor},
    types::Property,
};
use rand::Rng;

fn random_group_id<R: Rng + ?Sized>(rand: &mut R) -> String {
    let number_of_parts = rand.random_range(2..=4);

    let mut parts = Vec::with_capacity(number_of_parts);
    for _ in 0..number_of_parts {
        let part_length = rand.random_range(3..=10);
        let part: String = rand
            .sample_iter(&rand::distr::Alphanumeric)
            .take(part_length)
            .map(char::from)
            .collect();
        parts.push(part);
    }
    parts.join(".")
}

fn random_artifact_id<R: Rng + ?Sized>(rand: &mut R) -> String {
    let part_length = rand.random_range(3..=20);
    rand.sample_iter(&rand::distr::Alphanumeric)
        .take(part_length)
        .map(char::from)
        .collect()
}
fn random_dependency<R: Rng + ?Sized>(rand: &mut R) -> Dependency {
    let group_id = random_group_id(rand);
    let artifact_id = random_artifact_id(rand);
    let version = random_version(rand);

    Dependency {
        group_id,
        artifact_id,
        version: Some(Property::Literal(version)),
        depend_type: None,
        scope: None,
        classifier: None,
    }
}
fn random_url<R: Rng + ?Sized>(rand: &mut R) -> String {
    let protocol = if rand.random_range(0..=1) == 0 {
        "http"
    } else {
        "https"
    };
    let domain_length = rand.random_range(5..=15);
    let domain: String = rand
        .sample_iter(&rand::distr::Alphanumeric)
        .take(domain_length)
        .map(char::from)
        .collect();
    format!("{}://{}.com", protocol, domain)
}

fn random_repository<R: Rng + ?Sized>(rand: &mut R) -> Repository {
    let random_url = random_url(rand);

    let id = if rand.random_range(0..=1) == 0 {
        let id_length = rand.random_range(3..=10);

        let id: String = rand
            .sample_iter(&rand::distr::Alphanumeric)
            .take(id_length)
            .map(char::from)
            .collect();
        Some(id)
    } else {
        None
    };
    let name = if rand.random_range(0..=1) == 0 {
        let name_length = rand.random_range(3..=10);

        let name: String = rand
            .sample_iter(&rand::distr::Alphanumeric)
            .take(name_length)
            .map(char::from)
            .collect();
        Some(name)
    } else {
        None
    };

    Repository {
        id,
        name,
        url: random_url,
        ..Default::default()
    }
}

fn random_version<R: Rng + ?Sized>(rand: &mut R) -> String {
    let major = rand.random_range(1..=10);
    let minor = rand.random_range(0..=10);
    let patch = rand.random_range(0..=10);
    format!("{}.{}.{}", major, minor, patch)
}
#[test]
fn basic_pom_file_creation() -> anyhow::Result<()> {
    let mut rand = rand::rngs::ThreadRng::default();

    for _ in 0..100 {
        let mut pom = PomEditor::new_with_group_and_artifact(
            &random_group_id(&mut rand),
            &random_artifact_id(&mut rand),
        );

        pom.set_version(&random_version(&mut rand));
        let number_of_repositories = rand.random_range(1..=5);
        for _ in 0..number_of_repositories {
            let repository = random_repository(&mut rand);
            pom.add_or_update_repository(repository)?;
        }
        let number_of_dependencies = rand.random_range(1..=10);

        for _ in 0..number_of_dependencies {
            let dependency = random_dependency(&mut rand);
            pom.add_or_update_dependency(dependency)?;
        }

        let to_string = pom.write_to_str()?;

        let parsed_from_string = PomEditor::load_from_str(&to_string)?;

        assert_eq!(parsed_from_string.get_group_id(), pom.get_group_id());
        assert_eq!(parsed_from_string.get_artifact_id(), pom.get_artifact_id());
        assert_eq!(parsed_from_string.get_version(), pom.get_version());

        assert_eq!(
            parsed_from_string.get_dependencies()?,
            pom.get_dependencies()?
        );

        assert_eq!(
            parsed_from_string.get_repositories()?,
            pom.get_repositories()?
        );

        println!("Parsed pom: {:?}", parsed_from_string);
    }

    Ok(())
}
