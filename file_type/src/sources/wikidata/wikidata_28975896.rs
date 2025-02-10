use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975896: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_896,
        source_type: SourceType::Wikidata,
        name: "Spline Control File",
        extensions: &["spl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
