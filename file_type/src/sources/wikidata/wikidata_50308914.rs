use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50308914: FileType = FileType {
    file_format: &FileFormat {
        id: 50_308_914,
        source_type: SourceType::Wikidata,
        name: "OMNIC Spectral Data File",
        extensions: &["spa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
