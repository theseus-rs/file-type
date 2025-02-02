use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856844: FileFormat = FileFormat {
    id: 105_856_844,
    source_type: SourceType::Wikidata,
    name: "GTKWave Saved session",
    extensions: &["gtkw"],
    media_types: &["application/vnd.gtkwave-gtkw"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x2A, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
