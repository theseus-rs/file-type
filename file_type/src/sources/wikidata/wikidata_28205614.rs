use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205614: FileFormat = FileFormat {
    id: 28_205_614,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 2 Icon Mask",
    extensions: &["bmm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
