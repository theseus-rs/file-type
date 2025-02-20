use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121839915: FileType = FileType {
    file_format: &FileFormat {
        id: 121_839_915,
        source_type: SourceType::Wikidata,
        name: "Encapsulated PostScript File Format 2.1",
        extensions: &["eps", "epsf"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
