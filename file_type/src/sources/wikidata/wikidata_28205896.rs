use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205896: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_896,
        source_type: SourceType::Wikidata,
        name: "DESR VFF",
        extensions: &["vff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
