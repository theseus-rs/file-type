use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855515: FileFormat = FileFormat {
    id: 105_855_515,
    source_type: SourceType::Wikidata,
    name: "PC Animate Plus Frame F/X",
    extensions: &["ffx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x58, 0x7C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
