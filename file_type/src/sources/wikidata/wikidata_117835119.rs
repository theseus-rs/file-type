use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117835119: FileFormat = FileFormat {
    id: 117_835_119,
    source_type: SourceType::Wikidata,
    name: "Complete PC FAX/Portable file",
    extensions: &["cfp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
