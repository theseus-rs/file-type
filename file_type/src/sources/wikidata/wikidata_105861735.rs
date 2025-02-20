use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_735,
        source_type: SourceType::Wikidata,
        name: "MegaPaint plug-in Module",
        extensions: &["mod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0x4D, 0x4F, 0x44, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
