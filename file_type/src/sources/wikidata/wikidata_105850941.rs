use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850941: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_941,
        source_type: SourceType::Wikidata,
        name: "Torque GUI control (XML)",
        extensions: &["taml"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x47, 0x75, 0x69, 0x43, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
