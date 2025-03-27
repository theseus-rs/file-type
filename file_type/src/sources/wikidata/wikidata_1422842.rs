use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1422842: FileType = FileType {
    file_format: &FileFormat {
        id: 1_422_842,
        source_type: SourceType::Wikidata,
        name: "Hewlett-Packard Graphics Language",
        extensions: &[],
        media_types: &["application/vnd.hp-HPGL"],
        signatures: &[],
        related_formats: &[],
    },
};
