use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61774549: FileType = FileType {
    file_format: &FileFormat {
        id: 61_774_549,
        source_type: SourceType::Wikidata,
        name: "Indeo File Format",
        extensions: &["ivf"],
        media_types: &["application/x-extension-ivf"],
        signatures: &[],
        related_formats: &[],
    },
};
