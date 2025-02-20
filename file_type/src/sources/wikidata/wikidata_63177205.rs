use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63177205: FileType = FileType {
    file_format: &FileFormat {
        id: 63_177_205,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Macintosh, version 4",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
