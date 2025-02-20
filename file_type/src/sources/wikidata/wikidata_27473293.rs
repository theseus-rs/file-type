use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473293: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_293,
        source_type: SourceType::Wikidata,
        name: "CADRG Overview Image",
        extensions: &["ovr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
