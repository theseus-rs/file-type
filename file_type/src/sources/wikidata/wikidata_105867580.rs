use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867580: FileFormat = FileFormat {
    id: 105_867_580,
    puid: "wikidata/105867580",
    name: "Nero ISO Compilation",
    extensions: &["nri"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0E, 0x4E, 0x65, 0x72, 0x6F, 0x49, 0x53, 0x4F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
