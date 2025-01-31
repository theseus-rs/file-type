use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856121: FileFormat = FileFormat {
    id: 105_856_121,
    puid: "wikidata/105856121",
    name: "Device Tree Source (with rem)",
    extensions: &["dts"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
