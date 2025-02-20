use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_609977990: FileType = FileType {
    file_format: &FileFormat {
        id: 609_977_990,
        source_type: SourceType::Linguist,
        name: "RPGLE",
        extensions: &["rpgle", "sqlrpgle"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
