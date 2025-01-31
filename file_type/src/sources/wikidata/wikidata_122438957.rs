use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122438957: FileFormat = FileFormat {
    id: 122_438_957,
    puid: "wikidata/122438957",
    name: "TurboTax 2009 Tax Return",
    extensions: &["tax2009"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
