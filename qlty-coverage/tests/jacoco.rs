use qlty_coverage::parser::Jacoco;
use qlty_coverage::Parser;

#[test]
fn jacoco_results() {
    // Make sure that the <?xml version="1.0"?> tag is always right at the beginning of the string to avoid parsing errors
    let input = include_str!("fixtures/jacoco/sample.xml");

    let parsed_results = Jacoco::new().parse_text(input).unwrap();
    insta::assert_yaml_snapshot!(parsed_results, @r#"
    - path: be/apo/basic/Application.java
      hits:
        - "-1"
        - "-1"
        - "-1"
        - "-1"
        - "-1"
        - "-1"
        - "3"
        - "-1"
        - "-1"
        - "0"
        - "0"
    - path: be/apo/basic/rest/EchoService.java
      hits:
        - "-1"
        - "-1"
        - "-1"
        - "-1"
        - "-1"
        - "-1"
        - "-1"
        - "3"
        - "-1"
        - "-1"
        - "-1"
        - "0"
    - path: be/apo/basic/rest/model/Poney.java
      hits:
        - "-1"
        - "-1"
        - "0"
        - "-1"
        - "-1"
        - "0"
        - "-1"
        - "-1"
        - "0"
        - "-1"
        - "-1"
        - "-1"
        - "0"
        - "0"
        - "-1"
        - "-1"
        - "0"
    "#);
}
