use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50259511: FileFormat = FileFormat {
    id: 50_259_511,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Macro-Enabled Template, version 2013",
    extensions: &["vstm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
