use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117853051: FileFormat = FileFormat {
    id: 117_853_051,
    puid: "wikidata/117853051",
    name: "HiJaak Draw file",
    extensions: &["pdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
