use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862173: FileFormat = FileFormat {
    id: 105_862_173,
    source_type: SourceType::Wikidata,
    name: "The Holy Noise module",
    extensions: &["thn"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x52, 0x50, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
