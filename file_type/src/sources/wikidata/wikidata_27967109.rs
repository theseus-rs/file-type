use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967109: FileFormat = FileFormat {
    id: 27_967_109,
    source_type: SourceType::Wikidata,
    name: "1tracker module",
    extensions: &["1tm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x31, 0x74, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x20, 0x6D, 0x6F, 0x64,
                    0x75, 0x6C, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
