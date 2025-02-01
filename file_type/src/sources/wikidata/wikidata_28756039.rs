use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28756039: FileFormat = FileFormat {
    id: 28_756_039,
    puid: "wikidata/28756039",
    name: "FLA",
    extensions: &["fla"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
