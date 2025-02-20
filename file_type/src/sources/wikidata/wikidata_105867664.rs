use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867664: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_664,
        source_type: SourceType::Wikidata,
        name: "NetWars Shape",
        extensions: &["nsh"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x45, 0x43, 0x54, 0x4F, 0x52, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
