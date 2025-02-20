use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2175160977: FileType = FileType {
    file_format: &FileFormat {
        id: 2_175_160_977,
        source_type: SourceType::Iana,
        name: "ATF",
        extensions: &[],
        media_types: &["application/ATF"],
        signatures: &[],
        related_formats: &[],
    },
};
