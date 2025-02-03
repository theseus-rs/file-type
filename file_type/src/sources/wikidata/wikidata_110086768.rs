use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110086768: FileFormat = FileFormat {
    id: 110_086_768,
    source_type: SourceType::Wikidata,
    name: "Agisoft Project Archive",
    extensions: &["psz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
