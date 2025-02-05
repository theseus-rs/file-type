use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856895: FileFormat = FileFormat {
    id: 105_856_895,
    source_type: SourceType::Wikidata,
    name: "GenePix Array List",
    extensions: &["gal"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x54, 0x46, 0x09])],
            },
        }],
    }],
    related_formats: &[],
};
