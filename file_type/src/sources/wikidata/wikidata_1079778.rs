use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1079778: FileType = FileType {
    file_format: &FileFormat {
        id: 1_079_778,
        source_type: SourceType::Wikidata,
        name: "SIS",
        extensions: &["sis", "sisx"],
        media_types: &["application/vnd.symbian.install"],
        signatures: &[],
        related_formats: &[],
    },
};
