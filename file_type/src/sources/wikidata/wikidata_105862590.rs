use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862590: FileFormat = FileFormat {
    id: 105_862_590,
    source_type: SourceType::Wikidata,
    name: "Multiple Alignment Format",
    extensions: &["maf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x6D, 0x61, 0x66, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
