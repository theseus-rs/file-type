use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110098924: FileType = FileType {
    file_format: &FileFormat {
        id: 110_098_924,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Drawing, version 3",
        extensions: &["vsd", "vss", "vst"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
