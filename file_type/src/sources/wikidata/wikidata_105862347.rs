use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862347: FileFormat = FileFormat {
    id: 105_862_347,
    source_type: SourceType::Wikidata,
    name: "OctaMED MMD1 module",
    extensions: &["med", "mmd1"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x44, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
