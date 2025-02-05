use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866178: FileFormat = FileFormat {
    id: 105_866_178,
    source_type: SourceType::Wikidata,
    name: "Commodore 128 BASIC V7.1 program",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x13])],
            },
        }],
    }],
    related_formats: &[],
};
