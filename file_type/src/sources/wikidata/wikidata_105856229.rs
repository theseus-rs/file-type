use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856229: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_229,
        source_type: SourceType::Wikidata,
        name: "Dice C project (v3)",
        extensions: &["dice"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x52, 0x4F, 0x4A, 0x45, 0x43, 0x54, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
