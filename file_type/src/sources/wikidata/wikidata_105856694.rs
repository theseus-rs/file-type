use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856694: FileFormat = FileFormat {
    id: 105_856_694,
    puid: "wikidata/105856694",
    name: "Unified Scripture Format XML",
    extensions: &["usx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
