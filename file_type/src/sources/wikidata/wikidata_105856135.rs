use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856135: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_135,
        source_type: SourceType::Wikidata,
        name: "Cubic Player archive cache",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x43, 0x61, 0x63,
                        0x68, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
