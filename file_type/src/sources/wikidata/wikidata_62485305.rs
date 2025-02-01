use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62485305: FileFormat = FileFormat {
    id: 62_485_305,
    puid: "wikidata/62485305",
    name: "Navisworks Document, version 2010",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
