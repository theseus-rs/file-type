use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111440975: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_975,
        source_type: SourceType::Wikidata,
        name: "Visual Basic property PAGe file",
        extensions: &["pag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
