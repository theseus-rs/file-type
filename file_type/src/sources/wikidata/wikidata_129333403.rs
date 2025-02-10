use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129333403: FileType = FileType {
    file_format: &FileFormat {
        id: 129_333_403,
        source_type: SourceType::Wikidata,
        name: "Gettext catalog file",
        extensions: &["pot"],
        media_types: &["application/x-gettext", "text/gettext", "text/x-gettext"],
        signatures: &[],
        related_formats: &[],
    },
};
