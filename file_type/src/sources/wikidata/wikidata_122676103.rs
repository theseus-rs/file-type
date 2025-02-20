use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122676103: FileType = FileType {
    file_format: &FileFormat {
        id: 122_676_103,
        source_type: SourceType::Wikidata,
        name: "JASC Brush File",
        extensions: &["jbr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
