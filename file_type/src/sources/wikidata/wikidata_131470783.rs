use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131470783: FileFormat = FileFormat {
    id: 131_470_783,
    puid: "wikidata/131470783",
    name: "MetaImage Source file",
    extensions: &["mha"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
