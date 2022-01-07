use libfatigue::config::types::{FatigueTesterConfig, RunDuration};

#[derive(serde::Deserialize)]
struct Test1Properties {
    some: String,
    value: i32,
}

#[test]
pub fn test_config_deserialization() {
    let cfg = r#"
run:
    base_url: http://google.com
    duration:
        iteration:
            iterations: 5000
actions:
  - type: test1
    properties:
        some: a
        value: 1
  - type: test2
    properties:
        other: properties
        are: here
        is: true
    "#;

    let result: FatigueTesterConfig = serde_yaml::from_str(cfg).unwrap();

    assert_eq!(2, result.actions.len());
    assert_eq!("test1", result.actions[0].action_type);
    let test1: Test1Properties =
        serde_yaml::from_value(result.actions[0].action_properties.clone()).unwrap();

    assert_eq!("a", test1.some);
    assert_eq!(1, test1.value);

    #[allow(unreachable_patterns)]
    match result.run_info.duration {
        RunDuration::Iteration {
            warm_up,
            iterations,
        } => {
            assert_eq!(0, warm_up);
            assert_eq!(5000, iterations)
        }
        _ => {
            assert!(false, "wrong variant")
        }
    }
}
