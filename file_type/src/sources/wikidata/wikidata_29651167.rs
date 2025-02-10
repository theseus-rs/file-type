use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29651167: FileType = FileType {
    file_format: &FileFormat {
        id: 29_651_167,
        source_type: SourceType::Wikidata,
        name: "Personal Address Book",
        extensions: &["pab"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
