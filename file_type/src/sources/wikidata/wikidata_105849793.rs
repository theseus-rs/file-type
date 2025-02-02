use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849793: FileFormat = FileFormat {
    id: 105_849_793,
    source_type: SourceType::Wikidata,
    name: "Singer embroidery design",
    extensions: &["csd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7C, 0x4B, 0xC3, 0x74, 0xE1, 0xC8, 0x53, 0xA4, 0x79, 0xB9, 0x01, 0x1D, 0xFC,
                    0x4F, 0xDD, 0x13, 0x86, 0x3F, 0x38, 0x83, 0xC3, 0x6D, 0x45, 0x66, 0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
