use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135671687: FileType = FileType {
    file_format: &FileFormat {
        id: 135_671_687,
        source_type: SourceType::Wikidata,
        name: "Clip Studio name file",
        extensions: &["cnsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
