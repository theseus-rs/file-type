use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63082675: FileType = FileType {
    file_format: &FileFormat {
        id: 63_082_675,
        source_type: SourceType::Wikidata,
        name: "Waveform Audio (WAVEFORMATEXTENSIBLE)",
        extensions: &["wav", "wave"],
        media_types: &["audio/x-wav"],
        signatures: &[],
        related_formats: &[],
    },
};
