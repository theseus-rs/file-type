use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_98844104: FileFormat = FileFormat {
    id: 98_844_104,
    puid: "wikidata/98844104",
    name: "Grasshopper custom Layout",
    extensions: &["GHLAYOUT"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x46, 0x72, 0x61, 0x67, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x6E, 0x61, 0x6D,
                    0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
