use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66134841: FileType = FileType {
    file_format: &FileFormat {
        id: 66_134_841,
        source_type: SourceType::Wikidata,
        name: "ACCDA file format",
        extensions: &["accda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
