import os
import tomli
import yaml

script_dir = __file__

def go_up(current_dir, k):
    for _ in range(k):
        current_dir = os.path.dirname(current_dir)
    return current_dir

project_dir = go_up(script_dir, 3)

def get_project_version():
    cargo_toml_dir = os.path.join(project_dir, "./Cargo.toml")
    with open(cargo_toml_dir, mode="rb") as f:
        cargo_toml = tomli.load(f)

    return "v" + cargo_toml["package"]["version"]

def get_template():
    template_dir = os.path.join(project_dir, "./benchmark_results/template.yml")
    with open(template_dir, "r") as f:
        template = yaml.safe_load(f)
    
    return template

def get_tag(s):
    return "#" + "-".join(s.replace(".","").lower().split())

def generate_benchmark_results():
    version = get_project_version()
    result_dir = os.path.join(project_dir, f"./benchmark_results/{version}")
    plots_dir = os.path.join(result_dir, "./plots")
    if os.path.isdir(result_dir):
        raise Exception(f"Version {version} already has a report")
    
    os.mkdir(result_dir)
    os.mkdir(plots_dir)

    template = get_template()

    markdown_content = []
    table_of_content = []

    # Add title
    title = template["Title"]
    title = title.replace("${version}", version)
    tag = get_tag(title)
    markdown_content.append("# " + title)

    toc_markdown = f"- [{title}]({tag})"
    table_of_content.append(toc_markdown)

    # Add description
    description = template["Description"]
    markdown_content.append(description)
    markdown_content.append("")

    # Add tests
    test_markdown_content = []
    for benchmark_test in template["Tests"]:
        description = template["Tests"][benchmark_test]["Description"]
        src_svg_dir = os.path.join(project_dir, f"./target/criterion/{benchmark_test.lower()}/report/lines.svg")
        des_svg_dir = os.path.join(plots_dir, f"./{benchmark_test.lower()}.svg")

        src_svg = open(src_svg_dir, "r")
        des_svg = open(des_svg_dir, "w")
        for data in src_svg.readlines():
            data = data.replace("<svg ", "<svg style=\"background-color:white\" ")
            des_svg.writelines(data)

        svg_name = f"{'%20'.join(benchmark_test.lower().split())}.svg"
        tag = get_tag(benchmark_test)

        content = []
        content.append(f"## {benchmark_test}")
        content.append(f"**Description**: {description}")
        content.append(f"![img](./plots/{svg_name})")
        content.append("")

        toc_markdown = f"  - [{benchmark_test}]({tag})"
        table_of_content.append(toc_markdown)

        test_markdown_content += content
    
    markdown_content += table_of_content
    markdown_content += test_markdown_content

    with open(os.path.join(result_dir, "./result.md"), "w") as f:
        f.write("\n".join(markdown_content))
    
generate_benchmark_results()