use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3301166835: FileType = FileType {
    file_format: &FileFormat {
        id: 3_301_166_835,
        source_type: SourceType::Iana,
        name: "vnd.radiance",
        extensions: &[],
        media_types: &["image/vnd.radiance"],
        signatures: &[],
        related_formats: &[],
    },
};
