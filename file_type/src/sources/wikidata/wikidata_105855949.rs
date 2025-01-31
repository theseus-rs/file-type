use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855949: FileFormat = FileFormat {
    id: 105_855_949,
    puid: "wikidata/105855949",
    name: "DMIS input data",
    extensions: &["dmi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
