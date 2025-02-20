use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110098687: FileType = FileType {
    file_format: &FileFormat {
        id: 110_098_687,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Drawing, version 2",
        extensions: &["vsd", "vss", "vst"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
