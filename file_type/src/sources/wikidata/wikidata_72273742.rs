use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72273742: FileFormat = FileFormat {
    id: 72_273_742,
    puid: "wikidata/72273742",
    name: "TPN",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
