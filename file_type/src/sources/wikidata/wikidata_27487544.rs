use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487544: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_544,
        source_type: SourceType::Wikidata,
        name: "Shapefile codepage file",
        extensions: &["cpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
