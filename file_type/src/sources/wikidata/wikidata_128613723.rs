use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128613723: FileType = FileType {
    file_format: &FileFormat {
        id: 128_613_723,
        source_type: SourceType::Wikidata,
        name: "AspectJ file format",
        extensions: &["aj"],
        media_types: &["text/x-aspectj"],
        signatures: &[],
        related_formats: &[],
    },
};
