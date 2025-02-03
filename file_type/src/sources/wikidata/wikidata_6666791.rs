use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6666791: FileFormat = FileFormat {
    id: 6_666_791,
    source_type: SourceType::Wikidata,
    name: "Log ASCII Standard Format",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
