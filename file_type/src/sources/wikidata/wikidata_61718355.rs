use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61718355: FileType = FileType {
    file_format: &FileFormat {
        id: 61_718_355,
        source_type: SourceType::Wikidata,
        name: "Microsoft PowerPoint for Macintosh, version 4",
        extensions: &["ppt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
