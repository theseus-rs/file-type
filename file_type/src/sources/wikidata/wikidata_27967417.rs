use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967417: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_417,
        source_type: SourceType::Wikidata,
        name: "Callus OPL Register Log",
        extensions: &["cym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
