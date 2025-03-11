use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860692: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_692,
        source_type: SourceType::Wikidata,
        name: "rdiff network delta data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x73, 0x02, 0x36])],
                },
            }],
        }],
        related_formats: &[],
    },
};
