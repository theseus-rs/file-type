use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205604: FileFormat = FileFormat {
    id: 28_205_604,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 1 Icon Mask",
    extensions: &["msk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
