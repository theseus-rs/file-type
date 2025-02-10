use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_21104579: FileType = FileType {
    file_format: &FileFormat {
        id: 21_104_579,
        source_type: SourceType::Wikidata,
        name: "Free Lossless Image Format",
        extensions: &["flif"],
        media_types: &["image/flif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4C, 0x49, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
