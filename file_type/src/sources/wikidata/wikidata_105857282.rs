use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857282: FileFormat = FileFormat {
    id: 105_857_282,
    puid: "wikidata/105857282",
    name: "HEC-HMS Control specifications data",
    extensions: &["control"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
