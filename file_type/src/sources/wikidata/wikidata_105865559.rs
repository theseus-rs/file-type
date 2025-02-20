use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865559: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_559,
        source_type: SourceType::Wikidata,
        name: "PsychoPy 2 Experiment",
        extensions: &["psyexp"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x50, 0x73, 0x79, 0x63, 0x68, 0x6F, 0x50, 0x79, 0x32, 0x65, 0x78,
                        0x70, 0x65, 0x72, 0x69, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x76, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
