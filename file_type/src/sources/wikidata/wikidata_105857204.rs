use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857204: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_204,
        source_type: SourceType::Wikidata,
        name: "HandStory eBook",
        extensions: &["hsb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x01, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
