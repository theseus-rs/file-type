use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979406: FileFormat = FileFormat {
    id: 27_979_406,
    source_type: SourceType::Wikidata,
    name: "QTL",
    extensions: &["qtl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
