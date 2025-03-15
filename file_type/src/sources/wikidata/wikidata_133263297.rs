use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133263297: FileType = FileType {
    file_format: &FileFormat {
        id: 133_263_297,
        source_type: SourceType::Wikidata,
        name: "Daisy Talking Book Resource File 3",
        extensions: &["res"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
