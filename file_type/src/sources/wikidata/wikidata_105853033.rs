use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853033: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_033,
        source_type: SourceType::Wikidata,
        name: "Terragen Surface Map",
        extensions: &["srf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x45, 0x52, 0x52, 0x41, 0x47, 0x45, 0x4E, 0x53, 0x55, 0x52, 0x46,
                        0x4D, 0x41, 0x50, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
