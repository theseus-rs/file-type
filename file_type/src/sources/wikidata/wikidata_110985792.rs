use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110985792: FileType = FileType {
    file_format: &FileFormat {
        id: 110_985_792,
        source_type: SourceType::Wikidata,
        name: "Twin VQ format",
        extensions: &["vqf"],
        media_types: &["audio/x-twinvq"],
        signatures: &[],
        related_formats: &[],
    },
};
