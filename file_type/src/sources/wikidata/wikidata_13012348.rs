use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_13012348: FileFormat = FileFormat {
    id: 13_012_348,
    puid: "wikidata/13012348",
    name: "Adobe Flash project",
    extensions: &["fla"],
    media_types: &["application/vnd.adobe.fla"],
    internal_signatures: &[],
    related_formats: &[],
};
