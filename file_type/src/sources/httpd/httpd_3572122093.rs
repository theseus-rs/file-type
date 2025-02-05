use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3572122093: FileFormat = FileFormat {
    id: 3_572_122_093,
    source_type: SourceType::Httpd,
    name: "chipnuts karaoke mmd",
    extensions: &["mmd"],
    media_types: &["application/vnd.chipnuts.karaoke-mmd"],
    signatures: &[],
    related_formats: &[],
};
