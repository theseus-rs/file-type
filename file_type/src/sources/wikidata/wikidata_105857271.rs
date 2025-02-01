use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857271: FileFormat = FileFormat {
    id: 105_857_271,
    puid: "wikidata/105857271",
    name: "HydroCAD Project",
    extensions: &["hcp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x48, 0x79, 0x64, 0x72, 0x6F, 0x43, 0x41, 0x44, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
