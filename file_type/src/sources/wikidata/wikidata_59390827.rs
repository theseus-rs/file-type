use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59390827: FileFormat = FileFormat {
    id: 59_390_827,
    source_type: SourceType::Wikidata,
    name: "Domino XML Document Export",
    extensions: &["dxl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
