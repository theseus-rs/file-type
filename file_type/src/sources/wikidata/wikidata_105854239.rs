use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854239: FileFormat = FileFormat {
    id: 105_854_239,
    puid: "wikidata/105854239",
    name: "OptionSoft WindowsXCompressor archive",
    extensions: &["gcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x47, 0x43, 0x46, 0x00, 0x00, 0x00, 0x20, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
