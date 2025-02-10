use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1187861090: FileType = FileType {
    file_format: &FileFormat {
        id: 1_187_861_090,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.lpp",
        extensions: &[],
        media_types: &["application/vnd.3gpp.lpp"],
        signatures: &[],
        related_formats: &[],
    },
};
