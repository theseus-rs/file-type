use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853637: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_637,
        source_type: SourceType::Wikidata,
        name: "AGEMA File Format",
        extensions: &["aff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x46, 0x46, 0x00, 0x41, 0x47, 0x45, 0x4D, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
