use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127378208: FileFormat = FileFormat {
    id: 127_378_208,
    puid: "wikidata/127378208",
    name: "FreeBASIC source code file",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
