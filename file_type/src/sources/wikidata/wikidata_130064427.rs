use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130064427: FileType = FileType {
    file_format: &FileFormat {
        id: 130_064_427,
        source_type: SourceType::Wikidata,
        name: "Koka file format",
        extensions: &["kk"],
        media_types: &["text/x-koka"],
        signatures: &[],
        related_formats: &[],
    },
};
