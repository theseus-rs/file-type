use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856154: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_154,
        source_type: SourceType::Wikidata,
        name: "Real-Time Sound Processor II Data",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x54, 0x53, 0x50, 0x2E, 0x44, 0x41, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
