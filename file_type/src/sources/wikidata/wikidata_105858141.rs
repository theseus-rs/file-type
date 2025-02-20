use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858141: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_141,
        source_type: SourceType::Wikidata,
        name: "Aquarius Cassette tape image",
        extensions: &["caq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
