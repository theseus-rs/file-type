use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205330: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_330,
        source_type: SourceType::Wikidata,
        name: "Epson Raw Image Format",
        extensions: &["erf"],
        media_types: &["image/x-epson-erf", "image/x-raw-epson"],
        signatures: &[],
        related_formats: &[],
    },
};
