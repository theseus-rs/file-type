use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_99972520: FileType = FileType {
    file_format: &FileFormat {
        id: 99_972_520,
        source_type: SourceType::Wikidata,
        name: "OmniPage Pro Document 10",
        extensions: &["opd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
