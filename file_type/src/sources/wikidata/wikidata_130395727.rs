use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130395727: FileType = FileType {
    file_format: &FileFormat {
        id: 130_395_727,
        source_type: SourceType::Wikidata,
        name: "ODIN file format",
        extensions: &["odin"],
        media_types: &["text/odin"],
        signatures: &[],
        related_formats: &[],
    },
};
