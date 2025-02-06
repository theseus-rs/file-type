use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858488: FileFormat = FileFormat {
    id: 105_858_488,
    source_type: SourceType::Wikidata,
    name: "Tiny Stuff format bitmap (med-res anim)",
    extensions: &["tn5", "tny"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x07, 0x77])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x07, 0x77])],
                },
            }],
        },
    ],
    related_formats: &[],
};
