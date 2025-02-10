use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96143857: FileType = FileType {
    file_format: &FileFormat {
        id: 96_143_857,
        source_type: SourceType::Wikidata,
        name: "SurferGrid format",
        extensions: &["grd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
