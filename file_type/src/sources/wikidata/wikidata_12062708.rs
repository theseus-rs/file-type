use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_12062708: FileFormat = FileFormat {
    id: 12_062_708,
    source_type: SourceType::Wikidata,
    name: "CDW file format",
    extensions: &["cdw"],
    media_types: &["image/cdw"],
    internal_signatures: &[],
    related_formats: &[],
};
