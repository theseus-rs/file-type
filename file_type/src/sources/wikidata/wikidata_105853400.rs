use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853400: FileFormat = FileFormat {
    id: 105_853_400,
    source_type: SourceType::Wikidata,
    name: "MicroHelp Library",
    extensions: &["slb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x48, 0x65, 0x6C, 0x70, 0x20, 0x4C, 0x69, 0x62,
                    0x72, 0x61, 0x72, 0x79, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
