use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125916674: FileType = FileType {
    file_format: &FileFormat {
        id: 125_916_674,
        source_type: SourceType::Wikidata,
        name: "NEC Thermo Tracer Image File TH5100",
        extensions: &["tmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
