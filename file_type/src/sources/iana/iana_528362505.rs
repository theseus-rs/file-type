use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_528362505: FileType = FileType {
    file_format: &FileFormat {
        id: 528_362_505,
        source_type: SourceType::Iana,
        name: "vnd.evolv.ecig.settings",
        extensions: &[],
        media_types: &["application/vnd.evolv.ecig.settings"],
        signatures: &[],
        related_formats: &[],
    },
};
