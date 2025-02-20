use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857233: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_233,
        source_type: SourceType::Wikidata,
        name: "RoboHelp Topic Export",
        extensions: &["hpt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x6F, 0x62, 0x6F, 0x48, 0x65, 0x6C, 0x70, 0x20, 0x54, 0x6F, 0x70,
                        0x69, 0x63, 0x20, 0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x20, 0x66, 0x69,
                        0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
