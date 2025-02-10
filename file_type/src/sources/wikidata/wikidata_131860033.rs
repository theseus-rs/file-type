use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131860033: FileType = FileType {
    file_format: &FileFormat {
        id: 131_860_033,
        source_type: SourceType::Wikidata,
        name: "VPIC file format",
        extensions: &["vpc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
