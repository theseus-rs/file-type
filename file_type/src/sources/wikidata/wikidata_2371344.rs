use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2371344: FileType = FileType {
    file_format: &FileFormat {
        id: 2_371_344,
        source_type: SourceType::Wikidata,
        name: "TeX font metric",
        extensions: &["tfm"],
        media_types: &["application/x-tex-tfm"],
        signatures: &[],
        related_formats: &[],
    },
};
