use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3673584085: FileType = FileType {
    file_format: &FileFormat {
        id: 3_673_584_085,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.list-wbxml",
        extensions: &[],
        media_types: &["application/vnd.uplanet.list-wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
