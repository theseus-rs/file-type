use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59714459: FileType = FileType {
    file_format: &FileFormat {
        id: 59_714_459,
        source_type: SourceType::Wikidata,
        name: "Encapsulated PostScript File Format",
        extensions: &["eps", "epsf"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
