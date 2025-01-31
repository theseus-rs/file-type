use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126485053: FileFormat = FileFormat {
    id: 126_485_053,
    puid: "wikidata/126485053",
    name: "Omnis Sudio Project Library file",
    extensions: &["lbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
