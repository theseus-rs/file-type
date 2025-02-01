use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66146060: FileFormat = FileFormat {
    id: 66_146_060,
    puid: "wikidata/66146060",
    name: "InfoPath Form Template File",
    extensions: &["xsn"],
    media_types: &["application/vnd.ms-cab-compressed"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x43, 0x46, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
