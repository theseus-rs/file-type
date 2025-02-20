use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116941808: FileType = FileType {
    file_format: &FileFormat {
        id: 116_941_808,
        source_type: SourceType::Wikidata,
        name: "Ulead Template Extension",
        extensions: &["tpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
