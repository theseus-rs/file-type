use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859331: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_331,
        source_type: SourceType::Wikidata,
        name: "Quartus Simulator Setting File",
        extensions: &["ssf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x49, 0x4D, 0x55, 0x4C, 0x41, 0x54, 0x4F, 0x52, 0x5F, 0x53, 0x45,
                        0x54, 0x54, 0x49, 0x4E, 0x47, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
