use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73504589: FileFormat = FileFormat {
    id: 73_504_589,
    puid: "wikidata/73504589",
    name: "CorelFlow",
    extensions: &["cfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
