use crate::format::FileFormat;

pub(crate) const LINGUIST_407: FileFormat = FileFormat {
    id: 407,
    puid: "linguist/407",
    name: "YAML",
    extensions: &[
        "mir",
        "reek",
        "rviz",
        "sublime-syntax",
        "syntax",
        "yaml",
        "yaml-tmlanguage",
        "yaml.sed",
        "yml",
        "yml.mysql",
    ],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
