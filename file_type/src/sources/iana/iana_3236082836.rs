use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3236082836: FileType = FileType {
    file_format: &FileFormat {
        id: 3_236_082_836,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-dialog-group+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-dialog-group+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
