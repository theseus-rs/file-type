use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125858086: FileFormat = FileFormat {
    id: 125_858_086,
    puid: "wikidata/125858086",
    name: "Python GUI Source File",
    extensions: &["pyw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
