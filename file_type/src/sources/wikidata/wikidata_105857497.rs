use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857497: FileFormat = FileFormat {
    id: 105_857_497,
    source_type: SourceType::Wikidata,
    name: "3D Construction Kit Brushes data",
    extensions: &["3bd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x20, 0x43, 0x6F, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69,
                    0x6F, 0x6E, 0x20, 0x4B, 0x69, 0x74, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D,
                    0x20, 0x62, 0x72, 0x75, 0x73, 0x68, 0x65, 0x73, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
