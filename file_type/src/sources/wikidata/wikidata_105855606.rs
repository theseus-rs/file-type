use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855606: FileFormat = FileFormat {
    id: 105_855_606,
    source_type: SourceType::Wikidata,
    name: "osu! script (.osz ; .osu)",
    extensions: &["osu"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6F, 0x73, 0x75, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D,
                    0x61, 0x74, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
