use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904472: FileFormat = FileFormat {
    id: 29_904_472,
    source_type: SourceType::Wikidata,
    name: "ROOT",
    extensions: &["root"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x6F, 0x6F, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
