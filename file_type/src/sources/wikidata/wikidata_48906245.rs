use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48906245: FileFormat = FileFormat {
    id: 48_906_245,
    puid: "wikidata/48906245",
    name: "Harvard Graphics Vector Graphics",
    extensions: &["cht"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
