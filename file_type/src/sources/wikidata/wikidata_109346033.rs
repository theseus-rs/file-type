use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109346033: FileType = FileType {
    file_format: &FileFormat {
        id: 109_346_033,
        source_type: SourceType::Wikidata,
        name: "osu! beatmap archive (.osz)",
        extensions: &["osz"],
        media_types: &["application/x-osu-beatmap-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
