use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28551274: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_274,
        source_type: SourceType::Wikidata,
        name: "Adobe Arbitrary Map File",
        extensions: &["amp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
