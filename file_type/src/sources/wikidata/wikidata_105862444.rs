use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862444: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_444,
        source_type: SourceType::Wikidata,
        name: "Norton Commander module message (ENG)",
        extensions: &["msg"],
        media_types: &["application/x-norton-msg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x62, 0x6F, 0x72, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
