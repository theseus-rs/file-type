use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133829932: FileType = FileType {
    file_format: &FileFormat {
        id: 133_829_932,
        source_type: SourceType::Wikidata,
        name: "Picasso 64",
        extensions: &["p64"],
        media_types: &["image/x-picasso-64"],
        signatures: &[],
        related_formats: &[],
    },
};
