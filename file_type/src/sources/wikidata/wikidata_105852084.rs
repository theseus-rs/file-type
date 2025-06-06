use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852084: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_084,
        source_type: SourceType::Wikidata,
        name: "Multibit Bitcoin blockchain",
        extensions: &["spvchain"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x56, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
