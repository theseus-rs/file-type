use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_777561: FileFormat = FileFormat {
    id: 777_561,
    source_type: SourceType::Wikidata,
    name: "BinHex",
    extensions: &["hcx", "hex", "hqx"],
    media_types: &[
        "application/binhex",
        "application/mac-binhex",
        "application/mac-binhex40",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
