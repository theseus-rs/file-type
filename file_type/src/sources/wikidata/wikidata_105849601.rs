use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849601: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_601,
        source_type: SourceType::Wikidata,
        name: "Help File Contents",
        extensions: &["cnt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
