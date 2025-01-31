use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129998017: FileFormat = FileFormat {
    id: 129_998_017,
    puid: "wikidata/129998017",
    name: "JSON query and transformation language file format",
    extensions: &["jslt"],
    media_types: &["text/x-jslt"],
    internal_signatures: &[],
    related_formats: &[],
};
