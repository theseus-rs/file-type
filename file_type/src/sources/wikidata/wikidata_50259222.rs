use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50259222: FileType = FileType {
    file_format: &FileFormat {
        id: 50_259_222,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Macro-Enabled Drawing, version 2013",
        extensions: &["vsdm"],
        media_types: &["application/vnd.ms-visio.drawing.macroEnabled.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
