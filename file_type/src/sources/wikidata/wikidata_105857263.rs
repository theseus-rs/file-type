use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857263: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_263,
        source_type: SourceType::Wikidata,
        name: "KeyShot environment",
        extensions: &["hdz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x64, 0x7A, 0x6C, 0x75, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
