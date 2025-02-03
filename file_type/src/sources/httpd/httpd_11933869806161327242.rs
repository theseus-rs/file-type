use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11933869806161327242: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "wordperfect",
    extensions: &["wpd"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[],
};
