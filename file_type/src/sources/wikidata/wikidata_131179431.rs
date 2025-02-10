use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131179431: FileType = FileType {
    file_format: &FileFormat {
        id: 131_179_431,
        source_type: SourceType::Wikidata,
        name: "TableGen file format",
        extensions: &["td"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
