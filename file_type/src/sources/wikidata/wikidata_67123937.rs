use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67123937: FileFormat = FileFormat {
    id: 67_123_937,
    puid: "wikidata/67123937",
    name: "Print Artist business card file format",
    extensions: &["bc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
