use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862582: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_582,
        source_type: SourceType::Wikidata,
        name: "Mighty Draw drawing",
        extensions: &["mig"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x67, 0x68, 0x74, 0x79, 0x20, 0x44, 0x72, 0x61, 0x77, 0x20,
                        0x64, 0x72, 0x61, 0x77, 0x69, 0x6E, 0x67, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
