use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50259511: FileFormat = FileFormat {
    id: 50_259_511,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Macro-Enabled Template, version 2013",
    extensions: &["vstm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
