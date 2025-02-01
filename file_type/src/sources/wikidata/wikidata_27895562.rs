use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27895562: FileFormat = FileFormat {
    id: 27_895_562,
    puid: "wikidata/27895562",
    name: "ADX, version 4",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
