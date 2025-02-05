use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857282: FileFormat = FileFormat {
    id: 105_857_282,
    source_type: SourceType::Wikidata,
    name: "HEC-HMS Control specifications data",
    extensions: &["control"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
