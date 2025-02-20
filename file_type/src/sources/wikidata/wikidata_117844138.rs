use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117844138: FileType = FileType {
    file_format: &FileFormat {
        id: 117_844_138,
        source_type: SourceType::Wikidata,
        name: "Hayes JTFax file",
        extensions: &["jtf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
