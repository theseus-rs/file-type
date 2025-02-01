use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857800: FileFormat = FileFormat {
    id: 105_857_800,
    puid: "wikidata/105857800",
    name: "Microsoft Incremental Linker data",
    extensions: &["ilk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x4C, 0x69, 0x6E,
                    0x6B, 0x65, 0x72, 0x20, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
