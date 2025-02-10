use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856321: FileFormat = FileFormat {
    id: 105_856_321,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for DOS Document",
    extensions: &["dcx", "doc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0xBE, 0x00, 0x00, 0x00, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
