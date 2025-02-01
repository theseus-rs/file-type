use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919043: FileFormat = FileFormat {
    id: 28_919_043,
    puid: "wikidata/28919043",
    name: "Sony HDV",
    extensions: &["m2t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
