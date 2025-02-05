use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967444: FileFormat = FileFormat {
    id: 27_967_444,
    source_type: SourceType::Wikidata,
    name: "AutoDesk FLIC Animation",
    extensions: &["fli"],
    media_types: &["video/fli"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x11, 0xAF])],
            },
        }],
    }],
    related_formats: &[],
};
