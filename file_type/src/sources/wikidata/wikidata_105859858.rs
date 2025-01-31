use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859858: FileFormat = FileFormat {
    id: 105_859_858,
    puid: "wikidata/105859858",
    name: "AGS audio data",
    extensions: &["vox"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4C, 0x49, 0x42, 0x1A, 0x06, 0x14, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
