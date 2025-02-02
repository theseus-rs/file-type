use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205611: FileFormat = FileFormat {
    id: 28_205_611,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 1 Hot Icon",
    extensions: &["hot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
