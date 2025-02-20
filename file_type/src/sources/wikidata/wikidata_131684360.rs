use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131684360: FileType = FileType {
    file_format: &FileFormat {
        id: 131_684_360,
        source_type: SourceType::Wikidata,
        name: "Sierra IO System file format",
        extensions: &["exdg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
