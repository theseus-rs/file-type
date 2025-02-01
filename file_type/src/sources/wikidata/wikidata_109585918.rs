use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109585918: FileFormat = FileFormat {
    id: 109_585_918,
    puid: "wikidata/109585918",
    name: "Painter framestack file format",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
