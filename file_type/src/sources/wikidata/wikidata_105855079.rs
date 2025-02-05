use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855079: FileFormat = FileFormat {
    id: 105_855_079,
    source_type: SourceType::Wikidata,
    name: "Broderbund Mohawk game data archive",
    extensions: &["mhk"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x48, 0x57, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
