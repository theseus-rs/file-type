use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861518: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_518,
        source_type: SourceType::Wikidata,
        name: "LogicSim circuit (Amiga ver.)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x53, 0x49, 0x4D, 0x43, 0x49, 0x52, 0x43, 0x00, 0x00, 0x00, 0x05,
                        0x00, 0x00, 0x00, 0xB4,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
