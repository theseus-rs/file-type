use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3596396: FileType = FileType {
    file_format: &FileFormat {
        id: 3_596_396,
        source_type: SourceType::Wikidata,
        name: "MDX",
        extensions: &["mdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
