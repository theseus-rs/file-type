use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862702: FileFormat = FileFormat {
    id: 105_862_702,
    source_type: SourceType::Wikidata,
    name: "OctaMED MMD3 module",
    extensions: &["med", "mmd3"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x44, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
