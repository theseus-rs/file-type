use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854211: FileFormat = FileFormat {
    id: 105_854_211,
    source_type: SourceType::Wikidata,
    name: "PIM compressed archive (v2)",
    extensions: &["pim"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x4D, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
