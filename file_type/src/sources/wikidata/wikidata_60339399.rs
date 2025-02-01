use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60339399: FileFormat = FileFormat {
    id: 60_339_399,
    puid: "wikidata/60339399",
    name: "Open Project File",
    extensions: &["pod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
