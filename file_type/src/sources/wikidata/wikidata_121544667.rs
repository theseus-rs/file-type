use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121544667: FileFormat = FileFormat {
    id: 121_544_667,
    puid: "wikidata/121544667",
    name: "At Home 2009 Tax Return File",
    extensions: &["t09"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
