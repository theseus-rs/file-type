use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47012486: FileType = FileType {
    file_format: &FileFormat {
        id: 47_012_486,
        source_type: SourceType::Wikidata,
        name: "MultiMate Text File",
        extensions: &["dox", "fnx", "pat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
