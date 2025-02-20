use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132539986: FileType = FileType {
    file_format: &FileFormat {
        id: 132_539_986,
        source_type: SourceType::Wikidata,
        name: "Dual Continuum Properties",
        extensions: &["dprops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
