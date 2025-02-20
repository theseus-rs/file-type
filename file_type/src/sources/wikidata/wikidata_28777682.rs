use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28777682: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_682,
        source_type: SourceType::Wikidata,
        name: "mm",
        extensions: &["mm"],
        media_types: &["application/freemind", "application/x-freemind"],
        signatures: &[],
        related_formats: &[],
    },
};
