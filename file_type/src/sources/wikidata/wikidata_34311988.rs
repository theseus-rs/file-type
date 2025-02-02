use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34311988: FileFormat = FileFormat {
    id: 34_311_988,
    source_type: SourceType::Wikidata,
    name: "Shen script",
    extensions: &["shen"],
    media_types: &["application/x-shen", "text/x-shen"],
    internal_signatures: &[],
    related_formats: &[],
};
