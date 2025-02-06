use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856451: FileFormat = FileFormat {
    id: 105_856_451,
    source_type: SourceType::Wikidata,
    name: "Lotus Symphony Worksheet (V1)",
    extensions: &["wr1"],
    media_types: &["application/vnd.lotus-1-2-3"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x02, 0x00, 0x05, 0x04, 0x06, 0x00, 0x08, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
