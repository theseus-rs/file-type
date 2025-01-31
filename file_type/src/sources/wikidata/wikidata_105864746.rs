use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864746: FileFormat = FileFormat {
    id: 105_864_746,
    puid: "wikidata/105864746",
    name: "Palm WineMaster list",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x50, 0x63, 0x57, 0x62, 0x50, 0x62, 0x57, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
