use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857582: FileFormat = FileFormat {
    id: 105_857_582,
    source_type: SourceType::Wikidata,
    name: "MasterDOS v1.x bootable disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x13, 0x4D, 0x44, 0x4F, 0x53, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
