use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48625328: FileType = FileType {
    file_format: &FileFormat {
        id: 48_625_328,
        source_type: SourceType::Wikidata,
        name: "Encapsulated PostScript, v2",
        extensions: &["eps", "epsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
