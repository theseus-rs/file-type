use crate::format::FileFormat;

pub(crate) const LINGUIST_339: FileFormat = FileFormat {
    id: 339,
    puid: "linguist/339",
    name: "SaltStack",
    extensions: &["sls"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
