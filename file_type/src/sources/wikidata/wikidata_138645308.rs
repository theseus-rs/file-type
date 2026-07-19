use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138645308: FileType = FileType {
    file_format: &FileFormat {
        id: 138_645_308,
        source_type: SourceType::Wikidata,
        name: "Specctra Session File",
        extensions: &["ses"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
