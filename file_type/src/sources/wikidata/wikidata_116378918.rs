use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116378918: FileType = FileType {
    file_format: &FileFormat {
        id: 116_378_918,
        source_type: SourceType::Wikidata,
        name: "Approach Database File",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
