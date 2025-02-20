use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59608150: FileType = FileType {
    file_format: &FileFormat {
        id: 59_608_150,
        source_type: SourceType::Wikidata,
        name: "Microsoft Expression Media",
        extensions: &["ivc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
