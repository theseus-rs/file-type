use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862906: FileFormat = FileFormat {
    id: 105_862_906,
    puid: "wikidata/105862906",
    name: "Mirror II Emulation File",
    extensions: &["mef"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x0E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
