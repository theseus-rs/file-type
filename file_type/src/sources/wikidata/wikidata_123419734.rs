use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123419734: FileFormat = FileFormat {
    id: 123_419_734,
    puid: "wikidata/123419734",
    name: "StuffIt Zip Archive",
    extensions: &["zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
