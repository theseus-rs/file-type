use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854696: FileFormat = FileFormat {
    id: 105_854_696,
    puid: "wikidata/105854696",
    name: "Acclaim Skeleton File",
    extensions: &["asf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
