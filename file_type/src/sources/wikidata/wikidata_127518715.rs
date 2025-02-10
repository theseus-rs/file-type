use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127518715: FileType = FileType {
    file_format: &FileFormat {
        id: 127_518_715,
        source_type: SourceType::Wikidata,
        name: "Zephir source code file",
        extensions: &["zep"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
