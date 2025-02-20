use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_968474296: FileType = FileType {
    file_format: &FileFormat {
        id: 968_474_296,
        source_type: SourceType::Iana,
        name: "vnd.familysearch.gedcom",
        extensions: &[],
        media_types: &["text/vnd.familysearch.gedcom"],
        signatures: &[],
        related_formats: &[],
    },
};
