use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859264: FileFormat = FileFormat {
    id: 105_859_264,
    puid: "wikidata/105859264",
    name: "Nintendo Binary Revolution SEQuence",
    extensions: &["brseq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x45, 0x51])],
            },
        }],
    }],
    related_formats: &[],
};
