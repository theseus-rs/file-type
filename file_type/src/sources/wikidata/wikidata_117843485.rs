use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117843485: FileFormat = FileFormat {
    id: 117_843_485,
    puid: "wikidata/117843485",
    name: "Faxable PCX",
    extensions: &["fcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
