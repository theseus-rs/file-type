use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858690: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_690,
        source_type: SourceType::Wikidata,
        name: "Bannermania font",
        extensions: &["fnt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAB, 0xCD, 0x00, 0x05])],
                },
            }],
        }],
        related_formats: &[],
    },
};
