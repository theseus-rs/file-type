use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1096371228: FileType = FileType {
    file_format: &FileFormat {
        id: 1_096_371_228,
        source_type: SourceType::Iana,
        name: "vnd.IPTC.NITF",
        extensions: &[],
        media_types: &["text/vnd.IPTC.NITF"],
        signatures: &[],
        related_formats: &[],
    },
};
