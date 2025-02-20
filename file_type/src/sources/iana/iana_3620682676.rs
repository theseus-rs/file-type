use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3620682676: FileType = FileType {
    file_format: &FileFormat {
        id: 3_620_682_676,
        source_type: SourceType::Iana,
        name: "vnd.xmpie.dpkg",
        extensions: &[],
        media_types: &["application/vnd.xmpie.dpkg"],
        signatures: &[],
        related_formats: &[],
    },
};
