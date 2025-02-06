use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979531: FileFormat = FileFormat {
    id: 27_979_531,
    source_type: SourceType::Wikidata,
    name: "Binary Property List",
    extensions: &["plist"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x70, 0x6C, 0x69, 0x73, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
