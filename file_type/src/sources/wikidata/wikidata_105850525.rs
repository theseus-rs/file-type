use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850525: FileFormat = FileFormat {
    id: 105_850_525,
    puid: "wikidata/105850525",
    name: "PS Vita SCECAF crash report",
    extensions: &["caf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x43, 0x45, 0x43, 0x41, 0x46, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
