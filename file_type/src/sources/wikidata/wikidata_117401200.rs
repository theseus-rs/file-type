use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117401200: FileType = FileType {
    file_format: &FileFormat {
        id: 117_401_200,
        source_type: SourceType::Wikidata,
        name: "Asymetrix Toolbook File 6-11.5",
        extensions: &["sbk", "tbk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
