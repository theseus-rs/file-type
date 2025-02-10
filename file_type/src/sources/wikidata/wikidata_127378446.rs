use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127378446: FileType = FileType {
    file_format: &FileFormat {
        id: 127_378_446,
        source_type: SourceType::Wikidata,
        name: "GLSL file",
        extensions: &["glsl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
