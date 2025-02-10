use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975617: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_617,
        source_type: SourceType::Wikidata,
        name: "GNU Triangulated Surface",
        extensions: &["gts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
