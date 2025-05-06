use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134267519: FileType = FileType {
    file_format: &FileFormat {
        id: 134_267_519,
        source_type: SourceType::Wikidata,
        name: "WSL file",
        extensions: &["wsl"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
