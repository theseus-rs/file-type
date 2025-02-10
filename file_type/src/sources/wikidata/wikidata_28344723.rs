use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28344723: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_723,
        source_type: SourceType::Wikidata,
        name: "Turbo Pascal chain file",
        extensions: &["chn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
