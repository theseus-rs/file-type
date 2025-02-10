use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111395832: FileType = FileType {
    file_format: &FileFormat {
        id: 111_395_832,
        source_type: SourceType::Wikidata,
        name: "PhotoSuite Slide Show File",
        extensions: &["pzs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
