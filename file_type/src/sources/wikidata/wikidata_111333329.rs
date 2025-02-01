use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333329: FileFormat = FileFormat {
    id: 111_333_329,
    puid: "wikidata/111333329",
    name: "PSION A-law file",
    extensions: &["psion"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
