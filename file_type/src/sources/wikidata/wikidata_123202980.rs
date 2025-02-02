use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123202980: FileFormat = FileFormat {
    id: 123_202_980,
    source_type: SourceType::Wikidata,
    name: "Microsoft DV-AVI Video File",
    extensions: &["dv-avi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
