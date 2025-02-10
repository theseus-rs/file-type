use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_84087713: FileType = FileType {
    file_format: &FileFormat {
        id: 84_087_713,
        source_type: SourceType::Wikidata,
        name: "RootsMagic Database",
        extensions: &["rmgc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
