use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131081636: FileType = FileType {
    file_format: &FileFormat {
        id: 131_081_636,
        source_type: SourceType::Wikidata,
        name: "Snowball source code file",
        extensions: &["sbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
