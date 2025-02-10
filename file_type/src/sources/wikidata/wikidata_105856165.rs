use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856165: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_165,
        source_type: SourceType::Wikidata,
        name: "Windows Files And Settings Transfer data repository",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4D, 0x4F, 0x43, 0x43, 0x4D, 0x4F, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
