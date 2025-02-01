use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850899: FileFormat = FileFormat {
    id: 105_850_899,
    puid: "wikidata/105850899",
    name: "Kundo smart card exchange Format",
    extensions: &["ktf"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
