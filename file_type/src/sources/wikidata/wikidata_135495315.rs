use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135495315: FileType = FileType {
    file_format: &FileFormat {
        id: 135_495_315,
        source_type: SourceType::Wikidata,
        name: "Q135495315",
        extensions: &["ofip"],
        media_types: &["application/vnd.ofip"],
        signatures: &[],
        related_formats: &[],
    },
};
