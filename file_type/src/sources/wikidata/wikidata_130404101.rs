use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130404101: FileFormat = FileFormat {
    id: 130_404_101,
    source_type: SourceType::Wikidata,
    name: "Opa file format",
    extensions: &["opa"],
    media_types: &["text/x-opa"],
    internal_signatures: &[],
    related_formats: &[],
};
