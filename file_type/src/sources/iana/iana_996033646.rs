use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_996033646: FileType = FileType {
    file_format: &FileFormat {
        id: 996_033_646,
        source_type: SourceType::Iana,
        name: "vnd.mseq",
        extensions: &[],
        media_types: &["application/vnd.mseq"],
        signatures: &[],
        related_formats: &[],
    },
};
