use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864906: FileFormat = FileFormat {
    id: 105_864_906,
    puid: "wikidata/105864906",
    name: "Palm MobileDB database",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x4D, 0x64, 0x62, 0x31, 0x4D, 0x64, 0x62, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
