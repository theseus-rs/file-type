use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858182: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_182,
        source_type: SourceType::Wikidata,
        name: "GP32 Free eXecutable Encrypted",
        extensions: &["fxe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x78, 0x65, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
