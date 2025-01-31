use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850946: FileFormat = FileFormat {
    id: 105_850_946,
    puid: "wikidata/105850946",
    name: "TurboFM Compiler chiptune",
    extensions: &["tfc"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x46, 0x4D, 0x63, 0x6F, 0x6D, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
