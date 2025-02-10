use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856493: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_493,
        source_type: SourceType::Wikidata,
        name: "Lotus 123 Worksheet (V1J)",
        extensions: &["wj1"],
        media_types: &["application/vnd.lotus-1-2-3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x08, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
