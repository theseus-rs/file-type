use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966906: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_906,
        source_type: SourceType::Wikidata,
        name: "MDX",
        extensions: &["mdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
