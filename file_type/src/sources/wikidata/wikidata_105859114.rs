use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859114: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_114,
        source_type: SourceType::Wikidata,
        name: "LEADTOOLS ABC bitmap",
        extensions: &["abc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x42, 0x43, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
