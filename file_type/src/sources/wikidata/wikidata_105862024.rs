use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862024: FileFormat = FileFormat {
    id: 105_862_024,
    source_type: SourceType::Wikidata,
    name: "Telepaint Menu",
    extensions: &["mnu"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x32, 0x02, 0x00, 0x4D, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
