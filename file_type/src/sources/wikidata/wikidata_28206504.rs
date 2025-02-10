use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206504: FileFormat = FileFormat {
    id: 28_206_504,
    source_type: SourceType::Wikidata,
    name: "Lotus Manuscript graphics",
    extensions: &["bit", "rle"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
