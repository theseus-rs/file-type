use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2515690866: FileType = FileType {
    file_format: &FileFormat {
        id: 2_515_690_866,
        source_type: SourceType::Iana,
        name: "vnd.olpc-sugar",
        extensions: &[],
        media_types: &["application/vnd.olpc-sugar"],
        signatures: &[],
        related_formats: &[],
    },
};
