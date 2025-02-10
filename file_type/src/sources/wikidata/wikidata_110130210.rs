use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110130210: FileType = FileType {
    file_format: &FileFormat {
        id: 110_130_210,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Drawing, version 4",
        extensions: &["vsd", "vss", "vst", "vsw"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
