use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50259104: FileType = FileType {
    file_format: &FileFormat {
        id: 50_259_104,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Template, version 2013",
        extensions: &["vstx"],
        media_types: &["application/vnd.ms-visio.template.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
