use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1375997697: FileType = FileType {
    file_format: &FileFormat {
        id: 1_375_997_697,
        source_type: SourceType::Iana,
        name: "statuslist+cwt",
        extensions: &[],
        media_types: &["application/statuslist+cwt"],
        signatures: &[],
        related_formats: &[],
    },
};
