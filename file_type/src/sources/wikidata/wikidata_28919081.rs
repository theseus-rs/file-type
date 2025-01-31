use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919081: FileFormat = FileFormat {
    id: 28_919_081,
    puid: "wikidata/28919081",
    name: "Adobe Premiere Batch List",
    extensions: &["pbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
