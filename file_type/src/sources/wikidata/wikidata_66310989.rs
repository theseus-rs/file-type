use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
