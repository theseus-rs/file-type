use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131717063: FileFormat = FileFormat {
    id: 131_717_063,
    puid: "wikidata/131717063",
    name: "AVS UCD Binary/ASCII file format",
    extensions: &["inp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
