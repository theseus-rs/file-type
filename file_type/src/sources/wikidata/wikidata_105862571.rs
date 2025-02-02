use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862571: FileFormat = FileFormat {
    id: 105_862_571,
    source_type: SourceType::Wikidata,
    name: "Poser Material (V5)",
    extensions: &["mt5"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
