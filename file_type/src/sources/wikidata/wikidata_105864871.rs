use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864871: FileFormat = FileFormat {
    id: 105_864_871,
    puid: "wikidata/105864871",
    name: "Palm List database",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x54, 0x41, 0x4C, 0x53, 0x64, 0x62,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
