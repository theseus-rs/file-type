use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60558729: FileFormat = FileFormat {
    id: 60_558_729,
    puid: "wikidata/60558729",
    name: "ClarisWorks Painting, version 2",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
