use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_310539211: FileType = FileType {
    file_format: &FileFormat {
        id: 310_539_211,
        source_type: SourceType::Httpd,
        name: "ogg",
        extensions: &["oga", "ogg", "spx", "opus"],
        media_types: &["audio/ogg"],
        signatures: &[],
        related_formats: &[],
    },
};
