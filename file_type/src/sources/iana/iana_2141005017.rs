use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2141005017: FileType = FileType {
    file_format: &FileFormat {
        id: 2_141_005_017,
        source_type: SourceType::Iana,
        name: "vnd.imagemeter.folder+zip",
        extensions: &[],
        media_types: &["application/vnd.imagemeter.folder+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
