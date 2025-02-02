use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857626: FileFormat = FileFormat {
    id: 105_857_626,
    source_type: SourceType::Wikidata,
    name: "LibDisk disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x4B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
