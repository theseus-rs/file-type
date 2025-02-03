use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854935: FileFormat = FileFormat {
    id: 105_854_935,
    source_type: SourceType::Wikidata,
    name: "Abracadata drawing",
    extensions: &["aig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x62, 0x72, 0x61, 0x63, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x20, 0x64,
                    0x72, 0x61, 0x77, 0x69, 0x6E, 0x67, 0x1A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
