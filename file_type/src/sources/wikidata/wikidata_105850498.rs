use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850498: FileFormat = FileFormat {
    id: 105_850_498,
    source_type: SourceType::Wikidata,
    name: "NetCDF (network Common Data Form)",
    extensions: &["cdl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x65, 0x74, 0x63, 0x64, 0x66, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
