use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855686: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_686,
        source_type: SourceType::Wikidata,
        name: "Sequencer One song",
        extensions: &["one"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x6F, 0x6E, 0x67, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
