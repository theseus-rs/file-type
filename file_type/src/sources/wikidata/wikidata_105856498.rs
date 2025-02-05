use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856498: FileFormat = FileFormat {
    id: 105_856_498,
    source_type: SourceType::Wikidata,
    name: "VP-Planner Plus spreadsheet",
    extensions: &["wks"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x02, 0x00, 0x04, 0x04, 0xF3, 0x00, 0x02, 0x00, 0x00, 0x00, 0x06,
                    0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
