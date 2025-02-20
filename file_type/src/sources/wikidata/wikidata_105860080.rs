use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860080: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_080,
        source_type: SourceType::Wikidata,
        name: "BlueZone VT Display Keymap",
        extensions: &["vdk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x41, 0x46, 0x4D, 0x56, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
