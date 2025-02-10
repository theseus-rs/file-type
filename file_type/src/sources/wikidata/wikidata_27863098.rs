use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863098: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_098,
        source_type: SourceType::Wikidata,
        name: "3GPP2 file format",
        extensions: &["3g2"],
        media_types: &["audio/3gpp2", "video/3gpp2"],
        signatures: &[],
        related_formats: &[],
    },
};
