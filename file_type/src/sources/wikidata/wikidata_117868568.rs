use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117868568: FileType = FileType {
    file_format: &FileFormat {
        id: 117_868_568,
        source_type: SourceType::Wikidata,
        name: "Imavox TurboFax file",
        extensions: &["tbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
