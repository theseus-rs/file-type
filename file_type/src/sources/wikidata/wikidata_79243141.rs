use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79243141: FileFormat = FileFormat {
    id: 79_243_141,
    puid: "wikidata/79243141",
    name: "Affinity Design document",
    extensions: &["afdesign"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
