use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_914318960: FileFormat = FileFormat {
    id: 914_318_960,
    source_type: SourceType::Linguist,
    name: "JavaScript+ERB",
    extensions: &["js.erb"],
    media_types: &["application/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};
