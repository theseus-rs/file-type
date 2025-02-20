use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850089: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_089,
        source_type: SourceType::Wikidata,
        name: "TheC64 Config/Joystick/Mode settings (X)",
        extensions: &["cjm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
