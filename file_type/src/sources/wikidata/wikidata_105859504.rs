use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859504: FileFormat = FileFormat {
    id: 105_859_504,
    source_type: SourceType::Wikidata,
    name: "MM Video E-Mail",
    extensions: &["vem"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x69, 0x67, 0x68, 0x20, 0x4A, 0x50, 0x45, 0x47, 0x20, 0x44, 0x61, 0x74,
                    0x61, 0x20, 0x69, 0x6E, 0x20, 0x4D, 0x65, 0x6D, 0x6F, 0x72, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
