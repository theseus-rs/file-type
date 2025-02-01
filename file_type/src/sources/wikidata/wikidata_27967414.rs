use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967414: FileFormat = FileFormat {
    id: 27_967_414,
    puid: "wikidata/27967414",
    name: "Instrument Bank",
    extensions: &["ibk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
