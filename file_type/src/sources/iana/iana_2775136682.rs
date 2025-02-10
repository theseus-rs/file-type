use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2775136682: FileType = FileType {
    file_format: &FileFormat {
        id: 2_775_136_682,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-dialog-fax-sendrecv+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-dialog-fax-sendrecv+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
