use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473615: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_615,
        source_type: SourceType::Wikidata,
        name: "Band Interleaved by Line Image File",
        extensions: &["bil"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
