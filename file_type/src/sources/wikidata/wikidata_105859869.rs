use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859869: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_869,
        source_type: SourceType::Wikidata,
        name: "Knowledge Adventure MoVie video",
        extensions: &["mov"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x41, 0x4D, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
