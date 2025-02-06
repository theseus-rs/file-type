use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867629: FileFormat = FileFormat {
    id: 105_867_629,
    source_type: SourceType::Wikidata,
    name: "NEC JIS encoded file",
    extensions: &["nec"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
