use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919105: FileFormat = FileFormat {
    id: 28_919_105,
    puid: "wikidata/28919105",
    name: "Avid Log Exchange",
    extensions: &["ale"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x65, 0x61, 0x64, 0x69, 0x6E, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
