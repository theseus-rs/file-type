use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866150: FileFormat = FileFormat {
    id: 105_866_150,
    source_type: SourceType::Wikidata,
    name: "Palm JFile Pro database",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x66, 0x44, 0x62, 0x4A, 0x46, 0x69, 0x6C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
