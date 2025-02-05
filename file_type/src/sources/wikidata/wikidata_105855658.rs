use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855658: FileFormat = FileFormat {
    id: 105_855_658,
    source_type: SourceType::Wikidata,
    name: "Orbiter scenario",
    extensions: &["scn"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x45, 0x47, 0x49, 0x4E, 0x5F, 0x44, 0x45, 0x53, 0x43, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
