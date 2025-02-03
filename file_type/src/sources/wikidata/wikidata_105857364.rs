use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857364: FileFormat = FileFormat {
    id: 105_857_364,
    source_type: SourceType::Wikidata,
    name: "Jw_cad data",
    extensions: &["jwc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6A, 0x77, 0x5F, 0x63, 0x61, 0x64, 0x28, 0x63, 0x29, 0x64, 0x61, 0x74, 0x61,
                    0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
