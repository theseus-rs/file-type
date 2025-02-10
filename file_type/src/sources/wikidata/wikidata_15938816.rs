use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_15938816: FileType = FileType {
    file_format: &FileFormat {
        id: 15_938_816,
        source_type: SourceType::Wikidata,
        name: "MATLAB M-File",
        extensions: &["m"],
        media_types: &["text/matlab"],
        signatures: &[],
        related_formats: &[],
    },
};
