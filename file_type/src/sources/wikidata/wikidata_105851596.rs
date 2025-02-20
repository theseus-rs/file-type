use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851596: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_596,
        source_type: SourceType::Wikidata,
        name: "Skyland's Star game data",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3F, 0x63, 0x77, 0x31, 0x20, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
