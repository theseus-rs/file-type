use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857292: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_292,
        source_type: SourceType::Wikidata,
        name: "EnCase forensics Hash",
        extensions: &["hash"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x41, 0x53, 0x48, 0x0D, 0x0A, 0xFF, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
