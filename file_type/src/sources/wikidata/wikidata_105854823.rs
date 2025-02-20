use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854823: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_823,
        source_type: SourceType::Wikidata,
        name: "IMA4 encoded audio",
        extensions: &["ima4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x6D, 0x61, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
