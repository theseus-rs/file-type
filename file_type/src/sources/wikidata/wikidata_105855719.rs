use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855719: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_719,
        source_type: SourceType::Wikidata,
        name: "GNU gprof performance data",
        extensions: &["out"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x6D, 0x6F, 0x6E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
