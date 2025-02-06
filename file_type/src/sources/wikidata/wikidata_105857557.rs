use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857557: FileFormat = FileFormat {
    id: 105_857_557,
    source_type: SourceType::Wikidata,
    name: "Autodesk Inventor Project",
    extensions: &["ipj"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x3C, 0x00, 0x3F, 0x00, 0x78, 0x00, 0x6D, 0x00, 0x6C, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
