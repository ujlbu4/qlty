use qlty_analysis::code::File;
use qlty_analysis::code::{capture_by_name, capture_source, NodeFilter};
use std::collections::HashSet;
use tree_sitter::Node;

pub const QUERY_MATCH_LIMIT: usize = 1024;

pub fn count<'a>(source_file: &'a File, node: &Node<'a>, filter: &NodeFilter) -> usize {
    let language = source_file.language();
    let query = language.field_query();

    let mut query_cursor = tree_sitter::QueryCursor::new();
    query_cursor.set_match_limit(QUERY_MATCH_LIMIT as u32);

    let all_matches = query_cursor.matches(query, *node, source_file.contents.as_bytes());

    let mut fields = HashSet::new();

    for field_match in all_matches {
        let name = capture_source(query, "name", &field_match, source_file);
        let field_capture = capture_by_name(query, "field", &field_match);

        if filter.exclude(&field_capture.node) {
            continue;
        }

        if let Some(parent) = field_capture.node.parent() {
            // In some languages, field nodes appear within call nodes. We don't want to count those.
            if !language.call_nodes().contains(&parent.kind()) {
                fields.insert(name);
            }
        } else {
            fields.insert(name);
        }
    }

    fields.len()
}

#[cfg(test)]
mod test {
    use super::*;

    mod rust {
        use super::*;

        #[test]
        fn struct_declaration() {
            let source_file = File::from_string(
                "rust",
                r#"
                struct Foo {
                    bar: i32,
                    baz: i32
                }

                fn do_something() {
                    let foo = Foo { bar: 42, baz: 0 };
                    "{} {}", foo.bar, foo.baz);
                }
                "#,
            );
            assert_eq!(
                2,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn read() {
            let source_file = File::from_string(
                "rust",
                r#"
                self.foo;
                "#,
            );
            assert_eq!(
                1,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn write() {
            let source_file = File::from_string(
                "rust",
                r#"
                self.foo = 1;
                "#,
            );
            assert_eq!(
                1,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn unique() {
            let source_file = File::from_string(
                "rust",
                r#"
                self.foo = 1;
                self.foo;
                "#,
            );
            assert_eq!(
                1,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn multiple() {
            let source_file = File::from_string(
                "rust",
                r#"
                self.foo;
                self.bar;
                "#,
            );
            assert_eq!(
                2,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn ignore_collaborators() {
            let source_file = File::from_string(
                "rust",
                r#"
                other.foo = 1;
                other.bar;
                "#,
            );
            assert_eq!(
                0,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }
    }

    mod kotlin {
        use super::*;

        #[test]
        fn class_declaration() {
            let source_file = File::from_string(
                "kotlin",
                r#"
                class Shark {
                    var name: String = ""
                    var age: Int = 0
                }

                fun doSomething() {
                    val shark = Shark()
                    shark.name = "Sammy"
                    shark.age = 5
                    println(shark.name)
                    println(shark.age)
                }
            "#,
            );
            assert_eq!(
                2,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn read() {
            let source_file = File::from_string(
                "kotlin",
                r#"
                this.foo
            "#,
            );
            assert_eq!(
                1,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn write() {
            let source_file = File::from_string(
                "kotlin",
                r#"
                this.foo = 1
            "#,
            );
            assert_eq!(
                1,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn unique() {
            let source_file = File::from_string(
                "kotlin",
                r#"
                this.foo = 1
                this.foo
            "#,
            );
            assert_eq!(
                1,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn multiple() {
            let source_file = File::from_string(
                "kotlin",
                r#"
                this.foo
                this.bar
            "#,
            );
            assert_eq!(
                2,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }

        #[test]
        fn ignore_collaborators() {
            let source_file = File::from_string(
                "kotlin",
                r#"
                other.foo = 1
                other.bar
            "#,
            );
            assert_eq!(
                0,
                count(
                    &source_file,
                    &source_file.parse().root_node(),
                    &NodeFilter::empty()
                )
            );
        }
    }
}
