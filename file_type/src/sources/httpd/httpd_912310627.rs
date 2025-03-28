use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_912310627: FileType = FileType {
    file_format: &FileFormat {
        id: 912_310_627,
        source_type: SourceType::Httpd,
        name: "cmdf",
        extensions: &["cmdf"],
        media_types: &["chemical/x-cmdf"],
        signatures: &[],
        related_formats: &[],
    },
};
