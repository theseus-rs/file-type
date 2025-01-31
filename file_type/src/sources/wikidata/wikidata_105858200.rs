use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858200: FileFormat = FileFormat {
    id: 105_858_200,
    puid: "wikidata/105858200",
    name: "Real-Time Sound Processor II Effect",
    extensions: &["eff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x54, 0x53, 0x50, 0x2E, 0x45, 0x46, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
