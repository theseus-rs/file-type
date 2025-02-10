use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856133: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_133,
        source_type: SourceType::Wikidata,
        name: "Ease Word-Pro Document",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x50, 0x45, 0x52, 0x41, 0x1A, 0x05, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
