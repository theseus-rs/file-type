use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858152: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_152,
        source_type: SourceType::Wikidata,
        name: "HP Palmtop 100/200LX Icon",
        extensions: &["icn", "xbg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x01, 0x00, 0x2C, 0x00, 0x20, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
