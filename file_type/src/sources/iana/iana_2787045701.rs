use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2787045701: FileType = FileType {
    file_format: &FileFormat {
        id: 2_787_045_701,
        source_type: SourceType::Iana,
        name: "vnd.zbrush.pcx",
        extensions: &[],
        media_types: &["image/vnd.zbrush.pcx"],
        signatures: &[],
        related_formats: &[],
    },
};
