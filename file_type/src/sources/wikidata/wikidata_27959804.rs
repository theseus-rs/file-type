use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959804: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_804,
        source_type: SourceType::Wikidata,
        name: "Ableton Live Clip",
        extensions: &["alc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
