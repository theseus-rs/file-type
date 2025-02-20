use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66663053: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_053,
        source_type: SourceType::Wikidata,
        name: "eSuite Presentations Graphics",
        extensions: &["pg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
