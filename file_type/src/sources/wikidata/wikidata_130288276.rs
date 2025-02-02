use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130288276: FileFormat = FileFormat {
    id: 130_288_276,
    source_type: SourceType::Wikidata,
    name: "MYTHSAV",
    extensions: &["mythsav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
