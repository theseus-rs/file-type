use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3627282021: FileType = FileType {
    file_format: &FileFormat {
        id: 3_627_282_021,
        source_type: SourceType::Iana,
        name: "vnd.dvb.dvbj",
        extensions: &[],
        media_types: &["application/vnd.dvb.dvbj"],
        signatures: &[],
        related_formats: &[],
    },
};
