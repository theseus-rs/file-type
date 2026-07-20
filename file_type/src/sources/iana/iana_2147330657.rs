use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2147330657: FileType = FileType {
    file_format: &FileFormat {
        id: 2_147_330_657,
        source_type: SourceType::Iana,
        name: "soundfont",
        extensions: &[],
        media_types: &["audio/soundfont"],
        signatures: &[],
        related_formats: &[],
    },
};
