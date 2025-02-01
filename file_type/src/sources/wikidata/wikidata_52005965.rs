use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52005965: FileFormat = FileFormat {
    id: 52_005_965,
    puid: "wikidata/52005965",
    name: "Micrografx Draw, version 3",
    extensions: &["drw"],
    media_types: &["application/x-mgx-designer"],
    internal_signatures: &[],
    related_formats: &[],
};
