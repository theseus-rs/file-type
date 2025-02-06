use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855261: FileFormat = FileFormat {
    id: 105_855_261,
    source_type: SourceType::Wikidata,
    name: "AngelCode Bitmap Font (text)",
    extensions: &["fnt"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x6E, 0x66, 0x6F, 0x20, 0x66, 0x61, 0x63, 0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
