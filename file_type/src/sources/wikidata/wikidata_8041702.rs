use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_8041702: FileType = FileType {
    file_format: &FileFormat {
        id: 8_041_702,
        source_type: SourceType::Wikidata,
        name: "eXtended Binary",
        extensions: &["xb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
