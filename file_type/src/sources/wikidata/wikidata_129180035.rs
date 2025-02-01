use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129180035: FileFormat = FileFormat {
    id: 129_180_035,
    puid: "wikidata/129180035",
    name: "Fish shell script",
    extensions: &["fish"],
    media_types: &["application/x-fish"],
    internal_signatures: &[],
    related_formats: &[],
};
