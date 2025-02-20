use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975864: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_864,
        source_type: SourceType::Wikidata,
        name: "OOGL Object File Format",
        extensions: &["off"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
