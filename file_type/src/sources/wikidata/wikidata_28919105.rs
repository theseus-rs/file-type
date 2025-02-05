use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919105: FileFormat = FileFormat {
    id: 28_919_105,
    source_type: SourceType::Wikidata,
    name: "Avid Log Exchange",
    extensions: &["ale"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x65, 0x61, 0x64, 0x69, 0x6E, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
