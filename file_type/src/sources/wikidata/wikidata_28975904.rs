use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975904: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_904,
        source_type: SourceType::Wikidata,
        name: "Specified points for body pressure file",
        extensions: &["bpi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
