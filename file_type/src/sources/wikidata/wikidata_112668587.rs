use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112668587: FileFormat = FileFormat {
    id: 112_668_587,
    source_type: SourceType::Wikidata,
    name: "Lightscape Material",
    extensions: &["atr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
