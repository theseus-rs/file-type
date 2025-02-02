use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121741899: FileFormat = FileFormat {
    id: 121_741_899,
    source_type: SourceType::Wikidata,
    name: "TurboTax 2008 Tax Return",
    extensions: &["tax2008"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
