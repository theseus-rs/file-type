use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865021: FileFormat = FileFormat {
    id: 105_865_021,
    puid: "wikidata/105865021",
    name: "Photodex ProShow Workspace",
    extensions: &["ppr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x68, 0x6F, 0x74, 0x6F, 0x64, 0x65, 0x78, 0x28, 0x52, 0x29, 0x20, 0x50,
                    0x72, 0x6F, 0x53, 0x68, 0x6F, 0x77, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x57, 0x6F,
                    0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
