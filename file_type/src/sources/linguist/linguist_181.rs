use crate::format::FileFormat;

pub(crate) const LINGUIST_181: FileFormat = FileFormat {
    id: 181,
    puid: "linguist/181",
    name: "Java",
    extensions: &["jav", "java", "jsh"],
    media_types: &["text/x-java"],
    internal_signatures: &[],
    related_formats: &[],
};
