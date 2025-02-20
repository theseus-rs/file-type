use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63344866: FileType = FileType {
    file_format: &FileFormat {
        id: 63_344_866,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows, version 2000",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
