use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850018: FileFormat = FileFormat {
    id: 105_850_018,
    source_type: SourceType::Wikidata,
    name: "TheC64 Config/Joystick/Mode settings (V)",
    extensions: &["cjm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
