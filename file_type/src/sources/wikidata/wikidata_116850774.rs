use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116850774: FileType = FileType {
    file_format: &FileFormat {
        id: 116_850_774,
        source_type: SourceType::Wikidata,
        name: "Card File",
        extensions: &["dtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
