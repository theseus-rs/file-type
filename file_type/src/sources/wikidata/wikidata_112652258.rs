use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112652258: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_258,
        source_type: SourceType::Wikidata,
        name: "3ds Max Characters",
        extensions: &["chr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
