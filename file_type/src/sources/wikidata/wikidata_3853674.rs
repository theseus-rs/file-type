use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3853674: FileType = FileType {
    file_format: &FileFormat {
        id: 3_853_674,
        source_type: SourceType::Wikidata,
        name: "MDF format",
        extensions: &["mdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
