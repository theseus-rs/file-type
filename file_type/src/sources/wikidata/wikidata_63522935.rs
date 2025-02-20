use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63522935: FileType = FileType {
    file_format: &FileFormat {
        id: 63_522_935,
        source_type: SourceType::Wikidata,
        name: "Parametric Technology Pro/ENGINEER File Format",
        extensions: &["prt"],
        media_types: &["application/pro_eng"],
        signatures: &[],
        related_formats: &[],
    },
};
