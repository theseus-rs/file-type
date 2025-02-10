use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_878295361: FileType = FileType {
    file_format: &FileFormat {
        id: 878_295_361,
        source_type: SourceType::Iana,
        name: "vnd.desmume.movie",
        extensions: &[],
        media_types: &["application/vnd.desmume.movie"],
        signatures: &[],
        related_formats: &[],
    },
};
