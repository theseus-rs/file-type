use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56655440: FileFormat = FileFormat {
    id: 56_655_440,
    puid: "wikidata/56655440",
    name: "HDT",
    extensions: &["hdt"],
    media_types: &["application/vnd.hdt"],
    internal_signatures: &[],
    related_formats: &[],
};
