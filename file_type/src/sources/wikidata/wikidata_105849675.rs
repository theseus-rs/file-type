use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849675: FileFormat = FileFormat {
    id: 105_849_675,
    source_type: SourceType::Wikidata,
    name: "EXTRACAD drawing",
    extensions: &["cad"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x08, 0x00, 0x00, 0x00, 0x45, 0x78, 0x74, 0x72, 0x61, 0x43, 0x41, 0x44, 0x07,
                    0x00, 0x00, 0x00, 0x44, 0x52, 0x41, 0x57, 0x49, 0x4E, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
