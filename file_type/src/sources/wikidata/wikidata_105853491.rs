use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853491: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_491,
        source_type: SourceType::Wikidata,
        name: "HLGuard Z config file",
        extensions: &["zcfg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x55, 0x41, 0x2D, 0x63, 0x66, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
