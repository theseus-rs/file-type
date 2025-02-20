use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852973: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_973,
        source_type: SourceType::Wikidata,
        name: "Amiga Money Settings (v1)",
        extensions: &["sets"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x4D, 0x31, 0x53, 0x45, 0x54, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
