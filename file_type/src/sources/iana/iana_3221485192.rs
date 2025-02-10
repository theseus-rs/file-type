use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3221485192: FileType = FileType {
    file_format: &FileFormat {
        id: 3_221_485_192,
        source_type: SourceType::Iana,
        name: "vnd.afpc.modca-objectcontainer",
        extensions: &[],
        media_types: &["application/vnd.afpc.modca-objectcontainer"],
        signatures: &[],
        related_formats: &[],
    },
};
