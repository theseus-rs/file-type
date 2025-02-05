use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860813: FileFormat = FileFormat {
    id: 105_860_813,
    source_type: SourceType::Wikidata,
    name: "Tsunami Media game data archive",
    extensions: &["rlb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4D, 0x49, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
