use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52063276: FileFormat = FileFormat {
    id: 52_063_276,
    puid: "wikidata/52063276",
    name: "SAP Document",
    extensions: &["ali"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
