use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852973: FileFormat = FileFormat {
    id: 105_852_973,
    source_type: SourceType::Wikidata,
    name: "Amiga Money Settings (v1)",
    extensions: &["sets"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4D, 0x31, 0x53, 0x45, 0x54, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
