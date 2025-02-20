use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_77432664: FileType = FileType {
    file_format: &FileFormat {
        id: 77_432_664,
        source_type: SourceType::Wikidata,
        name: "InfoPath Template Part",
        extensions: &["xtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
