use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96081183: FileFormat = FileFormat {
    id: 96_081_183,
    source_type: SourceType::Wikidata,
    name: "SystemModeler model archive format",
    extensions: &["sma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
