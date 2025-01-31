use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852479: FileFormat = FileFormat {
    id: 105_852_479,
    puid: "wikidata/105852479",
    name: "SigmaNEST Shape",
    extensions: &["shp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x48, 0x41, 0x50, 0x45, 0x20, 0x20, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
