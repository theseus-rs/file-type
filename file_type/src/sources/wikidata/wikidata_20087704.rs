use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_20087704: FileFormat = FileFormat {
    id: 20_087_704,
    puid: "wikidata/20087704",
    name: "TST",
    extensions: &["tst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
