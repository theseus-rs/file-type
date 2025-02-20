use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111568387: FileType = FileType {
    file_format: &FileFormat {
        id: 111_568_387,
        source_type: SourceType::Wikidata,
        name: "Managed Object Format",
        extensions: &["mof"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
