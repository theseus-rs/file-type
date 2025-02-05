use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_694995663: FileFormat = FileFormat {
    id: 694_995_663,
    source_type: SourceType::Httpd,
    name: "cooltalk",
    extensions: &["ice"],
    media_types: &["x-conference/x-cooltalk"],
    signatures: &[],
    related_formats: &[],
};
