use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73675958: FileFormat = FileFormat {
    id: 73_675_958,
    puid: "wikidata/73675958",
    name: "3M Printscape",
    extensions: &["psc", "std"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
