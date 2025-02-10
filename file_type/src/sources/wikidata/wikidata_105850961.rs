use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850961: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_961,
        source_type: SourceType::Wikidata,
        name: "MSX Tape image",
        extensions: &["tsx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x54, 0x61, 0x70, 0x65, 0x21, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
