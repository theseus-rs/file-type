use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858645: FileFormat = FileFormat {
    id: 105_858_645,
    source_type: SourceType::Wikidata,
    name: "Chasm CEL bitmap",
    extensions: &["cel"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x19, 0x91])],
            },
        }],
    }],
    related_formats: &[],
};
