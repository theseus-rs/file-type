use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856925: FileFormat = FileFormat {
    id: 105_856_925,
    source_type: SourceType::Wikidata,
    name: "GVA/GVA2000 Author lecture",
    extensions: &["gdb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x44, 0x42, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
