use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117850827: FileType = FileType {
    file_format: &FileFormat {
        id: 117_850_827,
        source_type: SourceType::Wikidata,
        name: "OAZ Fax file",
        extensions: &["oaz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
