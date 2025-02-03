use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865815: FileFormat = FileFormat {
    id: 105_865_815,
    source_type: SourceType::Wikidata,
    name: "Commodore 64 BASIC V2 program",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x08])],
            },
        }],
    }],
    related_formats: &[],
};
