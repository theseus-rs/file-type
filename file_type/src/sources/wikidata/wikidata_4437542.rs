use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4437542: FileType = FileType {
    file_format: &FileFormat {
        id: 4_437_542,
        source_type: SourceType::Wikidata,
        name: "Direct Connect Hublist",
        extensions: &["dcls", "dclst", "xml.bz2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
