use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61990494: FileType = FileType {
    file_format: &FileFormat {
        id: 61_990_494,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Drawing, version 2003-2010",
        extensions: &["vsd"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
