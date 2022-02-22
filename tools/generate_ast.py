#!/usr/bin/python3

# pub enum Expr {
#     Binary(Binary),
# }

# pub struct Binary {
#     pub left: Box<Expr>,
#     pub operator: Token,
#     pub right: Box<Expr>,
# }

import sys

base_productions = {
    "Expr": [
      "Binary   : Expr left, Token operator, Expr right",
      "Grouping : Expr expression",
      "Literal  : Token value",
      "Unary    : Token operator, Expr right",
    ]
}


def define_visitor(f, base_name, types):
    f.write("pub trait Visitor {\n")
    f.write("    type R;\n")
    for node_type in types:
        node_type = node_type.strip().split(":")[0].strip()
        f.write("    fn visit_{} (&self, {}: &{}) -> Self::R;\n"
                .format(
                        node_type.lower(),
                        node_type.lower(),
                        node_type
                    ))
    f.write("}\n\n")


def main():
    if len(sys.argv) != 2:
        print("Usage: generate_ast.py <output_directory>")
        sys.exit(1)

    with open(sys.argv[1] + "/ast/node.rs", "w") as f:
        f.write("use crate::token::token::Token;\n\n")

        for base_class, productions in base_productions.items():
            f.write("pub enum {} {{\n".format(base_class))
            for production in productions:
                f.write("    {}({}),\n"
                        .format(
                            production.split(":")[0].strip(),
                            production.split(":")[0].strip()
                         )
                        )
            f.write("}\n\n")

            define_visitor(f, base_class, productions)

            for production in productions:
                (class_name, params) = \
                        (prod.strip() for prod in production.split(":")[:2])
                f.write("pub struct {} {{\n".format(class_name))
                for param in params.split(","):
                    ptr_type = param.strip().split(" ")[0].strip()
                    if ptr_type == "Expr":
                        ptr_type = "Box<Expr>"

                    f.write("    pub {}: {},\n".format(
                        param.strip().split(" ")[1].strip(),
                        ptr_type
                    ))
                f.write("}\n\n")

                # implement types using visitor patter
                f.write("impl {} {{\n".format(class_name))
                f.write("   pub fn accept<T: Visitor> ")
                f.write("(&self, visitor: &mut T) -> T::R {\n")
                f.write("       return visitor.visit_{}(&self);\n"
                        .format(class_name.lower()))
                f.write("   }\n")
                f.write("}\n\n")


if __name__ == "__main__":
    main()
