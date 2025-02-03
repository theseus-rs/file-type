use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205601: FileFormat = FileFormat {
    id: 28_205_601,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 1 Icon",
    extensions: &["icn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
