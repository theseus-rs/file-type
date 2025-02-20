use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63391433: FileType = FileType {
    file_format: &FileFormat {
        id: 63_391_433,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for DOS",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
