use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857989: FileFormat = FileFormat {
    id: 105_857_989,
    puid: "wikidata/105857989",
    name: "Infinity Engine exported player Character (v1.2)",
    extensions: &["chr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x52, 0x20, 0x56, 0x31, 0x2E, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
