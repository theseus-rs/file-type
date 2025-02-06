use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96081183: FileFormat = FileFormat {
    id: 96_081_183,
    source_type: SourceType::Wikidata,
    name: "SystemModeler model archive format",
    extensions: &["sma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
