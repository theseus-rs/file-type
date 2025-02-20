use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113577664: FileType = FileType {
    file_format: &FileFormat {
        id: 113_577_664,
        source_type: SourceType::Wikidata,
        name: "Philips/OptImage's Master tool",
        extensions: &["cd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
