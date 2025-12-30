use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1477978204: FileType = FileType {
    file_format: &FileFormat {
        id: 1_477_978_204,
        source_type: SourceType::Iana,
        name: "vnd.uic.dosipas.v2",
        extensions: &[],
        media_types: &["application/vnd.uic.dosipas.v2"],
        signatures: &[],
        related_formats: &[],
    },
};
