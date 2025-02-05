use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850622: FileFormat = FileFormat {
    id: 105_850_622,
    source_type: SourceType::Wikidata,
    name: "Cookeo recipe",
    extensions: &["cok"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4F, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
