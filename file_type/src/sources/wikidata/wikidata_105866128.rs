use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866128: FileFormat = FileFormat {
    id: 105_866_128,
    source_type: SourceType::Wikidata,
    name: "PokeyNoise chiptune",
    extensions: &["pn"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xFF, 0xE0, 0x02, 0xE1, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
