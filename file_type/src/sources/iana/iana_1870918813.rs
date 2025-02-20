use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1870918813: FileType = FileType {
    file_format: &FileFormat {
        id: 1_870_918_813,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-dialog-transform+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-dialog-transform+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
