use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853883: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_883,
        source_type: SourceType::Wikidata,
        name: "SoundSlimmer compressed audio",
        extensions: &["mpz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x05, 0x04, 0x05, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
