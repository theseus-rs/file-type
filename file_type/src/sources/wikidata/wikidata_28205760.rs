use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205760: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_760,
        source_type: SourceType::Wikidata,
        name: "Borland Graphics Interface image",
        extensions: &["bgi", "icn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
