use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857349: FileFormat = FileFormat {
    id: 105_857_349,
    source_type: SourceType::Wikidata,
    name: "Qt Jambi User Interface",
    extensions: &["jui"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
