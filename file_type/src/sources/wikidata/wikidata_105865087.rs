use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865087: FileFormat = FileFormat {
    id: 105_865_087,
    puid: "wikidata/105865087",
    name: "PyScripter Project (ASCII)",
    extensions: &["psproj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x79, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x65, 0x72, 0x5D, 0x0D,
                    0x0A, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
