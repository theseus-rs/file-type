use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28756261: FileFormat = FileFormat {
    id: 28_756_261,
    puid: "wikidata/28756261",
    name: "FIG",
    extensions: &["fig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
