use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110501140: FileType = FileType {
    file_format: &FileFormat {
        id: 110_501_140,
        source_type: SourceType::Wikidata,
        name: "Associated Signature Container Extended",
        extensions: &["asice", "sce"],
        media_types: &["application/vnd.etsi.asic-e+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
