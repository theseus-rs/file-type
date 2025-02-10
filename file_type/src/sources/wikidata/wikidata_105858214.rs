use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858214: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_214,
        source_type: SourceType::Wikidata,
        name: "EzCad Drawing",
        extensions: &["ezd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x00, 0x5A, 0x00, 0x43, 0x00, 0x41, 0x00, 0x44, 0x00, 0x55, 0x00,
                        0x4E, 0x00, 0x49, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
