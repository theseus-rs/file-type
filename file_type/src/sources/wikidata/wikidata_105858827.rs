use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858827: FileFormat = FileFormat {
    id: 105_858_827,
    source_type: SourceType::Wikidata,
    name: "Fastgraph Pixel Run Format bitmap",
    extensions: &["prf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x00, 0x41, 0x00, 0x53, 0x00, 0x54, 0x00, 0x47, 0x00, 0x52, 0x00, 0x41,
                    0x00, 0x46, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
