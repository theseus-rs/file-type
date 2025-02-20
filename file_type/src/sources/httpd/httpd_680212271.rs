use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_680212271: FileType = FileType {
    file_format: &FileFormat {
        id: 680_212_271,
        source_type: SourceType::Httpd,
        name: "fujixerox edmics rlc",
        extensions: &["rlc"],
        media_types: &["image/vnd.fujixerox.edmics-rlc"],
        signatures: &[],
        related_formats: &[],
    },
};
