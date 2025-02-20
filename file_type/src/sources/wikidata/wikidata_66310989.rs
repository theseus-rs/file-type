use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66310989: FileType = FileType {
    file_format: &FileFormat {
        id: 66_310_989,
        source_type: SourceType::Wikidata,
        name: "Pantry Files",
        extensions: &["pl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
