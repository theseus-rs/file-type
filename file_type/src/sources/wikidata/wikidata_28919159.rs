use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919159: FileFormat = FileFormat {
    id: 28_919_159,
    puid: "wikidata/28919159",
    name: "Standard ACIS Text",
    extensions: &["sat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
