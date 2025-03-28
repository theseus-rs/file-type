use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856353: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_353,
        source_type: SourceType::Wikidata,
        name: "NeoRAGEx savestate",
        extensions: &["dat1"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x52, 0x58, 0x53, 0x54, 0x41, 0x54, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
