use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52005776: FileType = FileType {
    file_format: &FileFormat {
        id: 52_005_776,
        source_type: SourceType::Wikidata,
        name: "Hewlett Packard Graphics Language format",
        extensions: &["hpgl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
