use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130223835: FileType = FileType {
    file_format: &FileFormat {
        id: 130_223_835,
        source_type: SourceType::Wikidata,
        name: "Lean 3 file format",
        extensions: &["lean"],
        media_types: &["text/x-lean", "text/x-lean3"],
        signatures: &[],
        related_formats: &[],
    },
};
