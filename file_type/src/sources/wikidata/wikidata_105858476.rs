use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858476: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_476,
        source_type: SourceType::Wikidata,
        name: "Open Publication Structure eBook",
        extensions: &["epub"],
        media_types: &["application/epub+zip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
