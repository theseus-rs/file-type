use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136468748: FileType = FileType {
    file_format: &FileFormat {
        id: 136_468_748,
        source_type: SourceType::Wikidata,
        name: "Dolby Atmos Master File",
        extensions: &["damf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
