use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858619: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_619,
        source_type: SourceType::Wikidata,
        name: "Funny Paint bitmap",
        extensions: &["fun"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x0A, 0xCF, 0xE2])],
                },
            }],
        }],
        related_formats: &[],
    },
};
