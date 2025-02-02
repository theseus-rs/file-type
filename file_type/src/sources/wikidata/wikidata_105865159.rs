use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865159: FileFormat = FileFormat {
    id: 105_865_159,
    source_type: SourceType::Wikidata,
    name: "Commodore VIC-20 BASIC V2 program (8K RAM expansion)",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x12])],
            },
        }],
    }],
    related_formats: &[],
};
