use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_981279644: FileType = FileType {
    file_format: &FileFormat {
        id: 981_279_644,
        source_type: SourceType::Iana,
        name: "vnd.dvb.ait",
        extensions: &[],
        media_types: &["application/vnd.dvb.ait"],
        signatures: &[],
        related_formats: &[],
    },
};
