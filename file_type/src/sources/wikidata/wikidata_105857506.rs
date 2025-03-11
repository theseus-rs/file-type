use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857506: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_506,
        source_type: SourceType::Wikidata,
        name: "3-D Professional camera",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x43, 0x41, 0x4D, 0x45, 0x52, 0x41, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
