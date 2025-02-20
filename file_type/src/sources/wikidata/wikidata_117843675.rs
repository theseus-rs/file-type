use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117843675: FileType = FileType {
    file_format: &FileFormat {
        id: 117_843_675,
        source_type: SourceType::Wikidata,
        name: "Wicat file",
        extensions: &["ged"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
