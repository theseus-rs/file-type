use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3499695519: FileType = FileType {
    file_format: &FileFormat {
        id: 3_499_695_519,
        source_type: SourceType::Iana,
        name: "geofeed+csv",
        extensions: &[],
        media_types: &["application/geofeed+csv"],
        signatures: &[],
        related_formats: &[],
    },
};
