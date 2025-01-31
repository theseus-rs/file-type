use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100301097: FileFormat = FileFormat {
    id: 100_301_097,
    puid: "wikidata/100301097",
    name: "Flow Charting PDQ format",
    extensions: &["pdq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
