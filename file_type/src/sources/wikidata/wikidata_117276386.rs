use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117276386: FileFormat = FileFormat {
    id: 117_276_386,
    puid: "wikidata/117276386",
    name: "Old Business Planner File",
    extensions: &["udf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
