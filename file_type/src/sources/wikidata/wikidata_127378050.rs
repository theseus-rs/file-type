use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127378050: FileFormat = FileFormat {
    id: 127_378_050,
    puid: "wikidata/127378050",
    name: "Pyrex Source Code File",
    extensions: &["pyx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
