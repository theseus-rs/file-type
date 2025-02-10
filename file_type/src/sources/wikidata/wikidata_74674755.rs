use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_74674755: FileType = FileType {
    file_format: &FileFormat {
        id: 74_674_755,
        source_type: SourceType::Wikidata,
        name: "SPSS Table Look",
        extensions: &["tlo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
