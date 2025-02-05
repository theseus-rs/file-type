use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967188: FileFormat = FileFormat {
    id: 27_967_188,
    source_type: SourceType::Wikidata,
    name: "Future Composer (BSI) module",
    extensions: &["bsi"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x55, 0x43, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
