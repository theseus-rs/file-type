use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866777: FileFormat = FileFormat {
    id: 105_866_777,
    source_type: SourceType::Wikidata,
    name: "Glyphs outline hashes",
    extensions: &["processedhashmap"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x0A, 0x27])],
            },
        }],
    }],
    related_formats: &[],
};
