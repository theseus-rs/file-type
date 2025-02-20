use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859714: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_714,
        source_type: SourceType::Wikidata,
        name: "VLBI Experiment",
        extensions: &["vex"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x58, 0x5F, 0x72, 0x65, 0x76, 0x20, 0x3D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
