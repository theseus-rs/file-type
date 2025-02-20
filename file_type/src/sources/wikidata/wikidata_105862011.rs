use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862011: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_011,
        source_type: SourceType::Wikidata,
        name: "UltraEdit Menu",
        extensions: &["in1", "mb0", "mb1", "mnu", "pb0", "pb1"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x20, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
