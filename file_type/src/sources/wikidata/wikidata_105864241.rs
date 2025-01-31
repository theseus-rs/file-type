use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864241: FileFormat = FileFormat {
    id: 105_864_241,
    puid: "wikidata/105864241",
    name: "Palm Quicksheet",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x61, 0x74, 0x61, 0x53, 0x70, 0x72, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
