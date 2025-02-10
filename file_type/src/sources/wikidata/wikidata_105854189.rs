use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854189: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_189,
        source_type: SourceType::Wikidata,
        name: "SEMONE compressed archive",
        extensions: &["one"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x45, 0x4D, 0x68, 0x29])],
                },
            }],
        }],
        related_formats: &[],
    },
};
