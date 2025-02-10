use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_68480995: FileType = FileType {
    file_format: &FileFormat {
        id: 68_480_995,
        source_type: SourceType::Wikidata,
        name: "Kingsoft PowerWord Dictionary",
        extensions: &["dic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
