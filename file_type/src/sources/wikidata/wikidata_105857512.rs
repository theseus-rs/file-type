use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857512: FileFormat = FileFormat {
    id: 105_857_512,
    source_type: SourceType::Wikidata,
    name: "Dalton's Disk Disintegrator Pro disk image",
    extensions: &["ddd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0xC9, 0xBF, 0xD0])],
            },
        }],
    }],
    related_formats: &[],
};
