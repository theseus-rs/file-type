use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600493: FileFormat = FileFormat {
    id: 28_600_493,
    puid: "wikidata/28600493",
    name: "DeltaVision",
    extensions: &["dv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
