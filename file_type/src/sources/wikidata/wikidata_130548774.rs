use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130548774: FileType = FileType {
    file_format: &FileFormat {
        id: 130_548_774,
        source_type: SourceType::Wikidata,
        name: "Qlik file format",
        extensions: &["qvs", "qvw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
