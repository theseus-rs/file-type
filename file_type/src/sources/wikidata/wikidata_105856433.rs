use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856433: FileFormat = FileFormat {
    id: 105_856_433,
    source_type: SourceType::Wikidata,
    name: "Webshots Image",
    extensions: &["wb1"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x57, 0x42, 0x42, 0x31, 0x31, 0x31, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
