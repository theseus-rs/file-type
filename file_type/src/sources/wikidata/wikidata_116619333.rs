use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116619333: FileType = FileType {
    file_format: &FileFormat {
        id: 116_619_333,
        source_type: SourceType::Wikidata,
        name: "Amiga SVX",
        extensions: &["svx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
