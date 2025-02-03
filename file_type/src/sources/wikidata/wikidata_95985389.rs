use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95985389: FileFormat = FileFormat {
    id: 95_985_389,
    source_type: SourceType::Wikidata,
    name: "Affymetrix CHP file format",
    extensions: &["chp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
