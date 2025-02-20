use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50259355: FileType = FileType {
    file_format: &FileFormat {
        id: 50_259_355,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Macro-Enabled Stencil, version 2013",
        extensions: &["vssm"],
        media_types: &["application/vnd.ms-visio.stencil.macroEnabled.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
