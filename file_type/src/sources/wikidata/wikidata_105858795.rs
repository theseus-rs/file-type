use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858795: FileFormat = FileFormat {
    id: 105_858_795,
    puid: "wikidata/105858795",
    name: "Photo Enote (Enot) external photo viewer settings",
    extensions: &["bws"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x48, 0x4F, 0x54, 0x4F, 0x56, 0x49, 0x45, 0x57, 0x45, 0x52, 0x53,
                    0x45, 0x54, 0x54, 0x49, 0x4E, 0x47, 0x53, 0x5D, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
