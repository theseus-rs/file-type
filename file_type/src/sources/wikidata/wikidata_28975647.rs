use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975647: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_647,
        source_type: SourceType::Wikidata,
        name: "POV-Ray RAW triangle format",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
