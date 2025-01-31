use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967088: FileFormat = FileFormat {
    id: 27_967_088,
    puid: "wikidata/27967088",
    name: "Electronic Arts MUS",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
