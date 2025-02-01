use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861186: FileFormat = FileFormat {
    id: 105_861_186,
    puid: "wikidata/105861186",
    name: "X-Plane Painted Line",
    extensions: &["lin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
