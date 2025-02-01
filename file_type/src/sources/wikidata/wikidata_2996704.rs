use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2996704: FileFormat = FileFormat {
    id: 2_996_704,
    puid: "wikidata/2996704",
    name: ".htpasswd",
    extensions: &["htpasswd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
