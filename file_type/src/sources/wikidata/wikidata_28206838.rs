use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206838: FileFormat = FileFormat {
    id: 28_206_838,
    puid: "wikidata/28206838",
    name: "Palm bitmap",
    extensions: &["palm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
