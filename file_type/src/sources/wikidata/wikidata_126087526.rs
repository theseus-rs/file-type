use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126087526: FileType = FileType {
    file_format: &FileFormat {
        id: 126_087_526,
        source_type: SourceType::Wikidata,
        name: "Husqvarna / Premier+ Embroidery Stitch File",
        extensions: &["vp4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
