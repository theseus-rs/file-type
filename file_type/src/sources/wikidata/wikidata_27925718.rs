use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27925718: FileType = FileType {
    file_format: &FileFormat {
        id: 27_925_718,
        source_type: SourceType::Wikidata,
        name: "DTED Level 1 Gazetteer Key file",
        extensions: &["key"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
