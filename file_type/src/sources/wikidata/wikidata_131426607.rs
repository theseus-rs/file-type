use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131426607: FileFormat = FileFormat {
    id: 131_426_607,
    puid: "wikidata/131426607",
    name: "Wren source code file format",
    extensions: &["wren"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
