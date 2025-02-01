use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131426238: FileFormat = FileFormat {
    id: 131_426_238,
    puid: "wikidata/131426238",
    name: "Whiley file format",
    extensions: &["whiley"],
    media_types: &["text/x-whiley"],
    internal_signatures: &[],
    related_formats: &[],
};
