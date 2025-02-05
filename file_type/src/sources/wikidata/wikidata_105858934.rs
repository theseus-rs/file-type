use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858934: FileFormat = FileFormat {
    id: 105_858_934,
    source_type: SourceType::Wikidata,
    name: "PCX bitmap (v2.8, palette)",
    extensions: &["pcx"],
    media_types: &["image/vnd.zbrush.pcx"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
