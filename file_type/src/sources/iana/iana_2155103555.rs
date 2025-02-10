use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2155103555: FileType = FileType {
    file_format: &FileFormat {
        id: 2_155_103_555,
        source_type: SourceType::Iana,
        name: "vnd.sbm.cid",
        extensions: &[],
        media_types: &["application/vnd.sbm.cid"],
        signatures: &[],
        related_formats: &[],
    },
};
