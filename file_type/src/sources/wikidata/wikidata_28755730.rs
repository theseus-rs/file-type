use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28755730: FileFormat = FileFormat {
    id: 28_755_730,
    puid: "wikidata/28755730",
    name: "FDB (Legacy Family Tree)",
    extensions: &["fdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
