use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34290425: FileFormat = FileFormat {
    id: 34_290_425,
    source_type: SourceType::Wikidata,
    name: "Statistical Package for the Social Sciences output file",
    extensions: &["spo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
