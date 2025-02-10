use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979390: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_390,
        source_type: SourceType::Wikidata,
        name: "Animatic Film",
        extensions: &["flm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x27, 0x18, 0x28, 0x18])],
                },
            }],
        }],
        related_formats: &[],
    },
};
