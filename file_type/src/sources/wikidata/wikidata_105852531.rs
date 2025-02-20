use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852531: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_531,
        source_type: SourceType::Wikidata,
        name: "Labeler (v3.0) / Labels Unlimited (v2.0) Style",
        extensions: &["sty"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x62, 0x65, 0x6C, 0x65, 0x72, 0x20, 0x56, 0x33, 0x2E, 0x30,
                        0x20, 0x53, 0x54, 0x59, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
