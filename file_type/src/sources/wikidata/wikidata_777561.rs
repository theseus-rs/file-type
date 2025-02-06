use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

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
    signatures: &[],
    related_formats: &[],
};
