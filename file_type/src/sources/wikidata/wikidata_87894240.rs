use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87894240: FileFormat = FileFormat {
    id: 87_894_240,
    puid: "wikidata/87894240",
    name: "Avery Label Pro Document 3",
    extensions: &["lpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
