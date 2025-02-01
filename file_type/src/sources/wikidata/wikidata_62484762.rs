use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62484762: FileFormat = FileFormat {
    id: 62_484_762,
    puid: "wikidata/62484762",
    name: "AccessData Custom Content Image, Encrypted version",
    extensions: &["ad1", "ad2", "ad3", "ad4", "ad5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
