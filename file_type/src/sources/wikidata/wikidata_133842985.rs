use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133842985: FileType = FileType {
    file_format: &FileFormat {
        id: 133_842_985,
        source_type: SourceType::Wikidata,
        name: "Quantum Paint file",
        extensions: &["pbx"],
        media_types: &["image/x-quantum-paint"],
        signatures: &[],
        related_formats: &[],
    },
};
