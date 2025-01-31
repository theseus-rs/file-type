use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127378389: FileFormat = FileFormat {
    id: 127_378_389,
    puid: "wikidata/127378389",
    name: "Genie source code file",
    extensions: &["gs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
