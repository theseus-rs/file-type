use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853117: FileFormat = FileFormat {
    id: 105_853_117,
    puid: "wikidata/105853117",
    name: "Psion SH3 Spreadsheet",
    extensions: &["spr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x50, 0x52, 0x45, 0x41, 0x44, 0x53, 0x48, 0x45, 0x45, 0x54, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
