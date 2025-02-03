use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855758: FileFormat = FileFormat {
    id: 105_855_758,
    source_type: SourceType::Wikidata,
    name: "RightWriter rule dictionary (v3-4 DOS)",
    extensions: &["dct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x69, 0x67, 0x68, 0x74, 0x57, 0x72, 0x69, 0x74, 0x65, 0x72, 0x20, 0x28,
                    0x52, 0x29, 0x20, 0x52, 0x75, 0x6C, 0x65, 0x20, 0x64, 0x69, 0x63, 0x74, 0x69,
                    0x6F, 0x6E, 0x61, 0x72, 0x79, 0x2E, 0x20, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
