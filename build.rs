fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("material-dark".into())
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);

    slint_build::compile_with_config("src/backends/slint/ui/appwindow.slint", config).unwrap();
}