use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116648074: FileFormat = FileFormat {
    id: 116_648_074,
    source_type: SourceType::Wikidata,
    name: "TopLevel Forms Document",
    extensions: &["tfm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
