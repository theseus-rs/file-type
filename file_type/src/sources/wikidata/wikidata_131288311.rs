use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131288311: FileFormat = FileFormat {
    id: 131_288_311,
    puid: "wikidata/131288311",
    name: "Transaction Execution Approval Language file format",
    extensions: &["teal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
