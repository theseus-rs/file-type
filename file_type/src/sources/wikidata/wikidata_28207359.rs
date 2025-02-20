use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207359: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_359,
        source_type: SourceType::Wikidata,
        name: "TAP",
        extensions: &["tap"],
        media_types: &["image/vnd.tencent.tap"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x41, 0x50, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
