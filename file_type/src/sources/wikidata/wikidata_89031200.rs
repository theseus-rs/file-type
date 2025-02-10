use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_89031200: FileType = FileType {
    file_format: &FileFormat {
        id: 89_031_200,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Project 2-5",
        extensions: &["psproj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
