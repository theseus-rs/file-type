use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29642901: FileFormat = FileFormat {
    id: 29_642_901,
    puid: "wikidata/29642901",
    name: "C header file",
    extensions: &["h", "hpp", "hxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
