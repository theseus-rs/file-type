use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3675216483: FileType = FileType {
    file_format: &FileFormat {
        id: 3_675_216_483,
        source_type: SourceType::Iana,
        name: "vnd.novadigm.EXT",
        extensions: &[],
        media_types: &["application/vnd.novadigm.EXT"],
        signatures: &[],
        related_formats: &[],
    },
};
