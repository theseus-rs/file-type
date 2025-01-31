use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52005776: FileFormat = FileFormat {
    id: 52_005_776,
    puid: "wikidata/52005776",
    name: "Hewlett Packard Graphics Language format",
    extensions: &["hpgl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
