use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_150: FileFormat = FileFormat {
    id: 150,
    source_type: SourceType::Linguist,
    name: "HTML+ERB",
    extensions: &["erb", "erb.deface", "rhtml"],
    media_types: &["application/x-erb"],
    signatures: &[],
    related_formats: &[],
};
