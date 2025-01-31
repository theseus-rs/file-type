use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919037: FileFormat = FileFormat {
    id: 28_919_037,
    puid: "wikidata/28919037",
    name: "Type-2 DV AVI",
    extensions: &["avi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
