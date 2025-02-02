use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849621: FileFormat = FileFormat {
    id: 105_849_621,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM Maverick's C0DER protected",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
