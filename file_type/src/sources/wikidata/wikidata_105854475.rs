use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854475: FileFormat = FileFormat {
    id: 105_854_475,
    source_type: SourceType::Wikidata,
    name: "abs spreadsheet (with rem)",
    extensions: &["abs"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x27, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D,
                    0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x41, 0x42, 0x56, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
