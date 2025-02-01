use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967351: FileFormat = FileFormat {
    id: 27_967_351,
    puid: "wikidata/27967351",
    name: "iTunes Music Library, XML variant",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
