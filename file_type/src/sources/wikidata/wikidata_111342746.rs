use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111342746: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_746,
        source_type: SourceType::Wikidata,
        name: "Creamware STS-series sampler program",
        extensions: &["sts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
