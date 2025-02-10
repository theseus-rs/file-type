use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855616: FileFormat = FileFormat {
    id: 105_855_616,
    source_type: SourceType::Wikidata,
    name: "Darksiders game data package",
    extensions: &["oppc", "opps3"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x42, 0x50, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
