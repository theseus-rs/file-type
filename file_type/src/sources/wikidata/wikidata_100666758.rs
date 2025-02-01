use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100666758: FileFormat = FileFormat {
    id: 100_666_758,
    puid: "wikidata/100666758",
    name: "Apple iWork Pages, version 9",
    extensions: &["pages"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
