use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853182: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_182,
        source_type: SourceType::Wikidata,
        name: "Sacred 2 save game",
        extensions: &["sacred2save"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x45, 0x58, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
