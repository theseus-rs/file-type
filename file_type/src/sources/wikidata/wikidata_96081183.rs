use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96081183: FileFormat = FileFormat {
    id: 96_081_183,
    puid: "wikidata/96081183",
    name: "SystemModeler model archive format",
    extensions: &["sma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
