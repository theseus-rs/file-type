use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857214: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_214,
        source_type: SourceType::Wikidata,
        name: "HYSYS Simulation Case",
        extensions: &["hsc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x69, 0x6D, 0x75, 0x6C, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x43, 0x61,
                        0x73, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
