use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3621718850: FileType = FileType {
    file_format: &FileFormat {
        id: 3_621_718_850,
        source_type: SourceType::Iana,
        name: "mtl",
        extensions: &[],
        media_types: &["model/mtl"],
        signatures: &[],
        related_formats: &[],
    },
};
