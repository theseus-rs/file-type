use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136400450: FileType = FileType {
    file_format: &FileFormat {
        id: 136_400_450,
        source_type: SourceType::Wikidata,
        name: "Android Archive file format",
        extensions: &["aar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
