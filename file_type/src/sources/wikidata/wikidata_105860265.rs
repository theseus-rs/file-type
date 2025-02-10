use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860265: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_265,
        source_type: SourceType::Wikidata,
        name: "RemoteKeys profile",
        extensions: &["rkp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x65, 0x6D, 0x6F, 0x74, 0x65, 0x4B, 0x65, 0x79, 0x73, 0x20, 0x50,
                        0x72, 0x6F, 0x66, 0x64, 0x61, 0x74, 0x20, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
