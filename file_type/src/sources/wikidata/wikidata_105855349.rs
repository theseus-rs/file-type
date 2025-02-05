use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855349: FileFormat = FileFormat {
    id: 105_855_349,
    source_type: SourceType::Wikidata,
    name: "Flatpack Repo",
    extensions: &["flatpakrepo"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x46, 0x6C, 0x61, 0x74, 0x70, 0x61, 0x6B, 0x20, 0x52, 0x65, 0x70, 0x6F,
                    0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
