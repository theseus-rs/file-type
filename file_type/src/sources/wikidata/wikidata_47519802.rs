use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47519802: FileType = FileType {
    file_format: &FileFormat {
        id: 47_519_802,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format (generic)",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
