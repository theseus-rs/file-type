use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48692225: FileType = FileType {
    file_format: &FileFormat {
        id: 48_692_225,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Drawing, version 2000",
        extensions: &["vsd"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
