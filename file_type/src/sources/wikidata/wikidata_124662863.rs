use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124662863: FileType = FileType {
    file_format: &FileFormat {
        id: 124_662_863,
        source_type: SourceType::Wikidata,
        name: "Digital Video Broadcasting; Subtitling systems",
        extensions: &["sub"],
        media_types: &["image/vnd.dvb.subtitle"],
        signatures: &[],
        related_formats: &[],
    },
};
