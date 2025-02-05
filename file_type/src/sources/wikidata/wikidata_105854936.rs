use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854936: FileFormat = FileFormat {
    id: 105_854_936,
    source_type: SourceType::Wikidata,
    name: "SFzip SoundFont compressed archive",
    extensions: &["sfz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x53, 0x46, 0x5A, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
