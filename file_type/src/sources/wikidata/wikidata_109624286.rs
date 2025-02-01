use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109624286: FileFormat = FileFormat {
    id: 109_624_286,
    puid: "wikidata/109624286",
    name: "WebPlus Essentials Site",
    extensions: &["wpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
