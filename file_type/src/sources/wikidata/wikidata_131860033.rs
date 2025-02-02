use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131860033: FileFormat = FileFormat {
    id: 131_860_033,
    source_type: SourceType::Wikidata,
    name: "VPIC file format",
    extensions: &["vpc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
