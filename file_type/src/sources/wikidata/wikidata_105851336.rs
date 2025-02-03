use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851336: FileFormat = FileFormat {
    id: 105_851_336,
    source_type: SourceType::Wikidata,
    name: "AmiAtlas Towns data",
    extensions: &["town"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x49, 0x47, 0x41, 0x5F, 0x41, 0x54, 0x4C, 0x41, 0x53, 0x5F, 0x54,
                    0x4F, 0x57, 0x4E, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
