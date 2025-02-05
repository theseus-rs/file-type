use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855690: FileFormat = FileFormat {
    id: 105_855_690,
    source_type: SourceType::Wikidata,
    name: "Molden OpenGL model",
    extensions: &["ogl"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4D, 0x4F, 0x4C, 0x44, 0x45, 0x4E, 0x4F, 0x47, 0x4C, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
