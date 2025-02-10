use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27826468: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_468,
        source_type: SourceType::Wikidata,
        name: "Cascading Style Sheets Level 2 Revision 1",
        extensions: &["css"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
