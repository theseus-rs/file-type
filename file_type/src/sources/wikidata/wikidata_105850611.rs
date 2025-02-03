use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850611: FileFormat = FileFormat {
    id: 105_850_611,
    source_type: SourceType::Wikidata,
    name: "BlueJ Class Context",
    extensions: &["ctxt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x42, 0x6C, 0x75, 0x65, 0x4A, 0x20, 0x63, 0x6C, 0x61, 0x73, 0x73, 0x20,
                    0x63, 0x6F, 0x6E, 0x74, 0x65, 0x78, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
