use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63344877: FileType = FileType {
    file_format: &FileFormat {
        id: 63_344_877,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for DOS",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
