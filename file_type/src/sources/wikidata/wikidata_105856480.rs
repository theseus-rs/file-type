use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856480: FileFormat = FileFormat {
    id: 105_856_480,
    source_type: SourceType::Wikidata,
    name: "Total Commander File system extension (plugin)",
    extensions: &["wfx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
