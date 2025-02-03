use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50259355: FileFormat = FileFormat {
    id: 50_259_355,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Macro-Enabled Stencil, version 2013",
    extensions: &["vssm"],
    media_types: &["application/vnd.ms-visio.stencil.macroEnabled.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
