use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7573795: FileFormat = FileFormat {
    id: 7_573_795,
    source_type: SourceType::Wikidata,
    name: "sparse image",
    extensions: &["sparseimage"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x70, 0x72, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
