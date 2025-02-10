use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600260: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_260,
        source_type: SourceType::Wikidata,
        name: "AWD",
        extensions: &["awd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
