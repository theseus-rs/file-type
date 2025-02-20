use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856663: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_663,
        source_type: SourceType::Wikidata,
        name: "Crazy Machines model",
        extensions: &["ucm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x43, 0x4D, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
