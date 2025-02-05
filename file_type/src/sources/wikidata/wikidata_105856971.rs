use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856971: FileFormat = FileFormat {
    id: 105_856_971,
    source_type: SourceType::Wikidata,
    name: "GUI Design Studio Project",
    extensions: &["gdp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x0A, 0x09, 0x43, 0x49, 0x44,
                    0x20, 0x3D, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
