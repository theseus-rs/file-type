use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61707565: FileType = FileType {
    file_format: &FileFormat {
        id: 61_707_565,
        source_type: SourceType::Wikidata,
        name: "Waveform Audio (WAVEFORMATEX)",
        extensions: &["wav", "wave"],
        media_types: &["audio/x-wav"],
        signatures: &[],
        related_formats: &[],
    },
};
