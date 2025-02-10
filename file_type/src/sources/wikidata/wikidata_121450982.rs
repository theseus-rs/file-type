use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_121450982: FileType = FileType {
    file_format: &FileFormat {
        id: 121_450_982,
        source_type: SourceType::Wikidata,
        name: "NTI CD Image",
        extensions: &["ncd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x88, 0xCD, 0x77, 0xCD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
