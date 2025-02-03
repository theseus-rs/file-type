use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852903: FileFormat = FileFormat {
    id: 105_852_903,
    source_type: SourceType::Wikidata,
    name: "Shanda Bambook eBook",
    extensions: &["snb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4E, 0x42, 0x50, 0x30, 0x30, 0x30, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
