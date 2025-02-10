use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3841727505: FileType = FileType {
    file_format: &FileFormat {
        id: 3_841_727_505,
        source_type: SourceType::Iana,
        name: "vnd.sealed.mht",
        extensions: &[],
        media_types: &["application/vnd.sealed.mht"],
        signatures: &[],
        related_formats: &[],
    },
};
