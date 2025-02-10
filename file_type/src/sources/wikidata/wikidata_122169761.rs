use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122169761: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_761,
        source_type: SourceType::Wikidata,
        name: "Domain Cached Credentials",
        extensions: &["dcc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
