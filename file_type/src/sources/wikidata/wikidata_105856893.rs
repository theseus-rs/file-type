use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856893: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_893,
        source_type: SourceType::Wikidata,
        name: "DexDrive memory card save game",
        extensions: &["gme"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x31, 0x32, 0x33, 0x2D, 0x34, 0x35, 0x36, 0x2D, 0x53, 0x54, 0x44, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
