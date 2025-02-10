use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2610229657: FileType = FileType {
    file_format: &FileFormat {
        id: 2_610_229_657,
        source_type: SourceType::Iana,
        name: "vnd.bbf.usp.msg",
        extensions: &[],
        media_types: &["application/vnd.bbf.usp.msg"],
        signatures: &[],
        related_formats: &[],
    },
};
