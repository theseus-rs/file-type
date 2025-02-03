use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28777705: FileFormat = FileFormat {
    id: 28_777_705,
    source_type: SourceType::Wikidata,
    name: "MyHeritage Family Tree Builder",
    extensions: &["zed"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
