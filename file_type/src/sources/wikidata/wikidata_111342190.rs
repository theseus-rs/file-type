use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342190: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_190,
        source_type: SourceType::Wikidata,
        name: "Avalon sample",
        extensions: &["smp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
