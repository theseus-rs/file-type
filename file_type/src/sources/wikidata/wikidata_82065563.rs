use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_82065563: FileFormat = FileFormat {
    id: 82_065_563,
    source_type: SourceType::Wikidata,
    name: "Euphoria Database System",
    extensions: &["edb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
