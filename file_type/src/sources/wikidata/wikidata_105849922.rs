use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849922: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_922,
        source_type: SourceType::Wikidata,
        name: "Frostbite Container of ASsets",
        extensions: &["cas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFA, 0xCE, 0x0F, 0xF0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
