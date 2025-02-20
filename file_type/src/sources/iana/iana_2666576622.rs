use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2666576622: FileType = FileType {
    file_format: &FileFormat {
        id: 2_666_576_622,
        source_type: SourceType::Iana,
        name: "vnd.rhetorex.32kadpcm",
        extensions: &[],
        media_types: &["audio/vnd.rhetorex.32kadpcm"],
        signatures: &[],
        related_formats: &[],
    },
};
