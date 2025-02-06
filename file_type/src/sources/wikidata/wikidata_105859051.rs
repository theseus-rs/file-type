use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859051: FileFormat = FileFormat {
    id: 105_859_051,
    source_type: SourceType::Wikidata,
    name: "FrameMaker Bitmapped screen Font",
    extensions: &["bfont"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x61, 0x6B, 0x65, 0x72, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x46,
                    0x6F, 0x6E, 0x74, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
