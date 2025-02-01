use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865079: FileFormat = FileFormat {
    id: 105_865_079,
    puid: "wikidata/105865079",
    name: "FreePascal compiled Unit",
    extensions: &["ppu"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x50, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
