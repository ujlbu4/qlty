use qlty_coverage::parser::Simplecov;
use qlty_coverage::Parser;

#[test]
fn simplecov_results() {
    let input = include_str!("fixtures/simplecov/sample.json");
    let parsed_results = Simplecov::new().parse_text(input).unwrap();

    insta::assert_yaml_snapshot!(parsed_results, @r#"
    - path: sample.rb
      hits:
        - "-1"
        - "1"
        - "1"
        - "1"
        - "1"
        - "-1"
        - "-1"
        - "1"
        - "1"
        - "-1"
        - "-1"
        - "1"
        - "1"
        - "0"
        - "-1"
        - "1"
        - "-1"
        - "-1"
        - "-1"
        - "-2"
        - "-2"
        - "-2"
        - "-2"
        - "-2"
        - "-1"
    - path: sample_2.rb
      hits:
        - "1"
        - "1"
        - "1"
        - "0"
        - "-1"
        - "-1"
        - "1"
        - "0"
        - "-1"
        - "-1"
        - "-1"
    "#);
}
