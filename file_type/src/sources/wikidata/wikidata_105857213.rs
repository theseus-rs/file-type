use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857213: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_213,
        source_type: SourceType::Wikidata,
        name: "HippoEDIT Workspace",
        extensions: &["hewsp"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x58, 0x4D, 0x4C, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x53, 0x65,
                        0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
