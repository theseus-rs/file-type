use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1516953976: FileType = FileType {
    file_format: &FileFormat {
        id: 1_516_953_976,
        source_type: SourceType::Iana,
        name: "vnd.ecowin.seriesupdate",
        extensions: &[],
        media_types: &["application/vnd.ecowin.seriesupdate"],
        signatures: &[],
        related_formats: &[],
    },
};
