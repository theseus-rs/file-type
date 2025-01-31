use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127785602: FileFormat = FileFormat {
    id: 127_785_602,
    puid: "wikidata/127785602",
    name: "MetaPost file",
    extensions: &["mp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
