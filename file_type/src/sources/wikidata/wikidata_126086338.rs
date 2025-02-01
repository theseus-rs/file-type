use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126086338: FileFormat = FileFormat {
    id: 126_086_338,
    puid: "wikidata/126086338",
    name: "IMF Package Packing List",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
