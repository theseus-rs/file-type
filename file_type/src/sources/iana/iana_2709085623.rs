use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2709085623: FileType = FileType {
    file_format: &FileFormat {
        id: 2_709_085_623,
        source_type: SourceType::Iana,
        name: "vnd.dvb.pfr",
        extensions: &[],
        media_types: &["application/vnd.dvb.pfr"],
        signatures: &[],
        related_formats: &[],
    },
};
