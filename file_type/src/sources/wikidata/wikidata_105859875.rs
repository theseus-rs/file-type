use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859875: FileFormat = FileFormat {
    id: 105_859_875,
    puid: "wikidata/105859875",
    name: "VCG graph",
    extensions: &["vcg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x67, 0x72, 0x61, 0x70, 0x68, 0x3A, 0x20, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
