use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_86450854: FileType = FileType {
    file_format: &FileFormat {
        id: 86_450_854,
        source_type: SourceType::Wikidata,
        name: "ASICS",
        extensions: &["asics"],
        media_types: &["application/vnd.etsi.asic-s+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
