use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131418585: FileFormat = FileFormat {
    id: 131_418_585,
    puid: "wikidata/131418585",
    name: "wdiff file format",
    extensions: &["wdiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
