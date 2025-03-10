use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857489: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_489,
        source_type: SourceType::Wikidata,
        name: "3-D Professional scene",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x53, 0x43, 0x45, 0x4E, 0x45, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
