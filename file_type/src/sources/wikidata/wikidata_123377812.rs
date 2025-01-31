use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123377812: FileFormat = FileFormat {
    id: 123_377_812,
    puid: "wikidata/123377812",
    name: "Lightwright Library file",
    extensions: &["lwb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
