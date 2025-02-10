use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66439311: FileType = FileType {
    file_format: &FileFormat {
        id: 66_439_311,
        source_type: SourceType::Wikidata,
        name: "Navy DIF",
        extensions: &["dif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
