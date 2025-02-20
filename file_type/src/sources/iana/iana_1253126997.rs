use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1253126997: FileType = FileType {
    file_format: &FileFormat {
        id: 1_253_126_997,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.remote-setup",
        extensions: &[],
        media_types: &["application/vnd.yamaha.remote-setup"],
        signatures: &[],
        related_formats: &[],
    },
};
