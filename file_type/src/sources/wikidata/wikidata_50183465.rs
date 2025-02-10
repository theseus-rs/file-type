use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50183465: FileType = FileType {
    file_format: &FileFormat {
        id: 50_183_465,
        source_type: SourceType::Wikidata,
        name: "AXD HTTP Handler File",
        extensions: &["axd"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
