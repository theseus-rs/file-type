use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_225: FileType = FileType {
    file_format: &FileFormat {
        id: 225,
        source_type: SourceType::Linguist,
        name: "MATLAB",
        extensions: &["m", "matlab"],
        media_types: &["text/x-octave"],
        signatures: &[],
        related_formats: &[],
    },
};
