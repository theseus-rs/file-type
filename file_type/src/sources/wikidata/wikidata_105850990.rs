use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850990: FileFormat = FileFormat {
    id: 105_850_990,
    puid: "wikidata/105850990",
    name: "SequoiaView directory Tree",
    extensions: &["tre"],
    media_types: &["text/x-tre"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6E, 0x6F, 0x64, 0x65, 0x3E, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2F, 0x2F, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
