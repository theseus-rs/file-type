use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865122: FileFormat = FileFormat {
    id: 105_865_122,
    source_type: SourceType::Wikidata,
    name: "Dac-Easy Word Printer",
    extensions: &["prt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x41, 0x4D, 0x45, 0x20, 0x3D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
