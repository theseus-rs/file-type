use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29904539: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_539,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System output file",
        extensions: &["lst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
