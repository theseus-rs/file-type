use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
