use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853437: FileFormat = FileFormat {
    id: 105_853_437,
    source_type: SourceType::Wikidata,
    name: "Mr. Backup Z64 ROM dump",
    extensions: &["z64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x80, 0x37, 0x12, 0x40, 0x00, 0x00, 0x00, 0x0F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
