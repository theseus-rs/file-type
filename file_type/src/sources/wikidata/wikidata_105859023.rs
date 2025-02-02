use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859023: FileFormat = FileFormat {
    id: 105_859_023,
    source_type: SourceType::Wikidata,
    name: "STOS source",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x6F, 0x6E, 0x70, 0x6F, 0x75, 0x6C, 0x6F, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
