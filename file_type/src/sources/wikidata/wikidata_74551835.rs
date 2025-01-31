use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74551835: FileFormat = FileFormat {
    id: 74_551_835,
    puid: "wikidata/74551835",
    name: "ChiWriter Screen Font",
    extensions: &["sft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
