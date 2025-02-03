use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110995868: FileFormat = FileFormat {
    id: 110_995_868,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Half-fold Card File format",
    extensions: &["hcr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
