use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128205532: FileFormat = FileFormat {
    id: 128_205_532,
    puid: "wikidata/128205532",
    name: "UDO source code file",
    extensions: &["udo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
