use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1934041989: FileFormat = FileFormat {
    id: 1_934_041_989,
    source_type: SourceType::Httpd,
    name: "ms xpsdocument",
    extensions: &["xps"],
    media_types: &["application/vnd.ms-xpsdocument"],
    signatures: &[],
    related_formats: &[],
};
