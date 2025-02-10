use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4153744506: FileType = FileType {
    file_format: &FileFormat {
        id: 4_153_744_506,
        source_type: SourceType::Iana,
        name: "vnd.dvb.ipdcroaming",
        extensions: &[],
        media_types: &["application/vnd.dvb.ipdcroaming"],
        signatures: &[],
        related_formats: &[],
    },
};
