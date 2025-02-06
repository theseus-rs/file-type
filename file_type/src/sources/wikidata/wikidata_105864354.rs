use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864354: FileFormat = FileFormat {
    id: 105_864_354,
    source_type: SourceType::Wikidata,
    name: "Print Magic Card",
    extensions: &["pmc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x43, 0x41, 0x52, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
