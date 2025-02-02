use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855805: FileFormat = FileFormat {
    id: 105_855_805,
    source_type: SourceType::Wikidata,
    name: "Delphi Package",
    extensions: &["dpk"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x70, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
