use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850403: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_403,
        source_type: SourceType::Wikidata,
        name: "TheC64 Config/Joystick/Mode settings (J)",
        extensions: &["cjm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
