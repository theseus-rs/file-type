use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855998: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_998,
        source_type: SourceType::Wikidata,
        name: "Dis bytecode (not signed)",
        extensions: &["dis"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x0C, 0x80, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
