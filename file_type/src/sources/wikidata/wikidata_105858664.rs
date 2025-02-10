use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858664: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_664,
        source_type: SourceType::Wikidata,
        name: "Bitstream Compressed Outline font",
        extensions: &["bco"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x31, 0x8F, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
