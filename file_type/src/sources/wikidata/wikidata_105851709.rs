use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851709: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_709,
        source_type: SourceType::Wikidata,
        name: "Frostbite SuperBundle",
        extensions: &["sb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0xD1, 0xCE, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
