use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851234: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_234,
        source_type: SourceType::Wikidata,
        name: "Enterprise Music Box tune",
        extensions: &["tun"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x42, 0x31, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
