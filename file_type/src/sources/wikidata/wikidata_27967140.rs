use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967140: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_140,
        source_type: SourceType::Wikidata,
        name: "Digitrakker instrument",
        extensions: &["ist"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x49, 0x53, 0x54, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
