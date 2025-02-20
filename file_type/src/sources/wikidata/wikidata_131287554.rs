use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131287554: FileType = FileType {
    file_format: &FileFormat {
        id: 131_287_554,
        source_type: SourceType::Wikidata,
        name: "tcsh script file format",
        extensions: &["csh", "tcsh"],
        media_types: &["application/x-csh"],
        signatures: &[],
        related_formats: &[],
    },
};
