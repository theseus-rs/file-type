use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109265635: FileFormat = FileFormat {
    id: 109_265_635,
    puid: "wikidata/109265635",
    name: "Pro Write Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
