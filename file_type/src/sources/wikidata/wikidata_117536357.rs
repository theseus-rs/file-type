use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117536357: FileType = FileType {
    file_format: &FileFormat {
        id: 117_536_357,
        source_type: SourceType::Wikidata,
        name: "ArcSoft Album and SlideShow Files",
        extensions: &["abm", "sld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
