use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1215761775: FileType = FileType {
    file_format: &FileFormat {
        id: 1_215_761_775,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.pic-bw-large",
        extensions: &[],
        media_types: &["application/vnd.3gpp.pic-bw-large"],
        signatures: &[],
        related_formats: &[],
    },
};
