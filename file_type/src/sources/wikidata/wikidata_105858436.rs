use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858436: FileFormat = FileFormat {
    id: 105_858_436,
    source_type: SourceType::Wikidata,
    name: "Keyhole - Google Earth Overlay",
    extensions: &["eta"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4B, 0x65, 0x79, 0x68, 0x6F, 0x6C, 0x65, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
