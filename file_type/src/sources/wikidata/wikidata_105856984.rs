use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856984: FileFormat = FileFormat {
    id: 105_856_984,
    puid: "wikidata/105856984",
    name: "Spectrum Global Tracker chiptune",
    extensions: &["gtr"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x54, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
