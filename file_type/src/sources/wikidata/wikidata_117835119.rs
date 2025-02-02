use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117835119: FileFormat = FileFormat {
    id: 117_835_119,
    source_type: SourceType::Wikidata,
    name: "Complete PC FAX/Portable file",
    extensions: &["cfp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
