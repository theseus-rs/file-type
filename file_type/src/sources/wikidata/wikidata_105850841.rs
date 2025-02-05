use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850841: FileFormat = FileFormat {
    id: 105_850_841,
    source_type: SourceType::Wikidata,
    name: "Sony Ericsson remote control configuration",
    extensions: &["kcf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
