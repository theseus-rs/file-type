use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34747567: FileFormat = FileFormat {
    id: 34_747_567,
    source_type: SourceType::Wikidata,
    name: "Stronghold GM1",
    extensions: &["gm1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
