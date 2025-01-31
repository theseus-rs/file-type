use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7271574: FileFormat = FileFormat {
    id: 7_271_574,
    puid: "wikidata/7271574",
    name: "Quetzal file format",
    extensions: &["glksave", "sav"],
    media_types: &["application/x-glksave", "application/x-glksave"],
    internal_signatures: &[],
    related_formats: &[],
};
