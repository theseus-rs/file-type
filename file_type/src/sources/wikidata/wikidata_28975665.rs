use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975665: FileFormat = FileFormat {
    id: 28_975_665,
    puid: "wikidata/28975665",
    name: "3D Studio Max ASCII Export Format",
    extensions: &["ase"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x33, 0x44, 0x53, 0x4D, 0x41, 0x58, 0x5F, 0x41, 0x53, 0x43, 0x49, 0x49,
                    0x45, 0x58, 0x50, 0x4F, 0x52, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
