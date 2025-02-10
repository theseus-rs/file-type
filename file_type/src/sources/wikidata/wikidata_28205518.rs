use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205518: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_518,
        source_type: SourceType::Wikidata,
        name: "atomix.scores",
        extensions: &["scores"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
