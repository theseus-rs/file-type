use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_30311736: FileType = FileType {
    file_format: &FileFormat {
        id: 30_311_736,
        source_type: SourceType::Wikidata,
        name: "Alias Matte file",
        extensions: &["matte"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
