use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29209126: FileType = FileType {
    file_format: &FileFormat {
        id: 29_209_126,
        source_type: SourceType::Wikidata,
        name: "VirtualBox Disk Image",
        extensions: &["vdi"],
        media_types: &["application/x-vdi-disk", "application/x-virtualbox-vdi"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x3C, 0x3C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
