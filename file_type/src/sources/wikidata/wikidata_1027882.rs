use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1027882: FileType = FileType {
    file_format: &FileFormat {
        id: 1_027_882,
        source_type: SourceType::Wikidata,
        name: "Short Payment Descriptor",
        extensions: &["spayd"],
        media_types: &["application/x-shortpaymentdescriptor"],
        signatures: &[],
        related_formats: &[],
    },
};
