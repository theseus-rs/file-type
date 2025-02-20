use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850018: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_018,
        source_type: SourceType::Wikidata,
        name: "TheC64 Config/Joystick/Mode settings (V)",
        extensions: &["cjm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
