use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_993720141: FileType = FileType {
    file_format: &FileFormat {
        id: 993_720_141,
        source_type: SourceType::Iana,
        name: "vnd.dvb.subtitle",
        extensions: &[],
        media_types: &["text/vnd.dvb.subtitle"],
        signatures: &[],
        related_formats: &[],
    },
};
