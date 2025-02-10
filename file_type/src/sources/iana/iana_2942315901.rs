use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2942315901: FileType = FileType {
    file_format: &FileFormat {
        id: 2_942_315_901,
        source_type: SourceType::Iana,
        name: "vnd.dpgraph",
        extensions: &[],
        media_types: &["application/vnd.dpgraph"],
        signatures: &[],
        related_formats: &[],
    },
};
