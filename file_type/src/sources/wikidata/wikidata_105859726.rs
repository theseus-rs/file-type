use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859726: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_726,
        source_type: SourceType::Wikidata,
        name: "vCard - Business Card",
        extensions: &["vcard", "vcf"],
        media_types: &["text/vcard"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
