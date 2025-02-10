use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72000076: FileType = FileType {
    file_format: &FileFormat {
        id: 72_000_076,
        source_type: SourceType::Wikidata,
        name: "File Express Index Header",
        extensions: &["ixh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
