use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852332: FileFormat = FileFormat {
    id: 105_852_332,
    source_type: SourceType::Wikidata,
    name: "BML3MK5 snapshot",
    extensions: &["l3r"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x53, 0x55, 0x4D, 0x45, 0x5F, 0x42, 0x4D, 0x4C, 0x33, 0x4D, 0x4B,
                    0x35,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
