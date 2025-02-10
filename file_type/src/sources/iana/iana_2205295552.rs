use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2205295552: FileType = FileType {
    file_format: &FileFormat {
        id: 2_205_295_552,
        source_type: SourceType::Iana,
        name: "vnd.dvb.ipdcesgaccess",
        extensions: &[],
        media_types: &["application/vnd.dvb.ipdcesgaccess"],
        signatures: &[],
        related_formats: &[],
    },
};
