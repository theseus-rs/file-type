use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861731: FileFormat = FileFormat {
    id: 105_861_731,
    source_type: SourceType::Wikidata,
    name: "MagicEngine savestate",
    extensions: &["me1"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0x45, 0x43, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
