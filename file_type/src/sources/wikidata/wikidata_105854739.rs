use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854739: FileFormat = FileFormat {
    id: 105_854_739,
    puid: "wikidata/105854739",
    name: "Asura engine Resources package (generic)",
    extensions: &["asr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x73, 0x75, 0x72, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
