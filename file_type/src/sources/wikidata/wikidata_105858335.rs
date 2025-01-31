use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858335: FileFormat = FileFormat {
    id: 105_858_335,
    puid: "wikidata/105858335",
    name: "Adobe Edge Project",
    extensions: &["edge"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
