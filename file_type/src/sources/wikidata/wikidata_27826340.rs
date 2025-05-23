use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27826340: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_340,
        source_type: SourceType::Wikidata,
        name: "Auxiliary file, AUX variant",
        extensions: &["aux"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
