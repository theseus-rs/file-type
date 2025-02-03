use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112652534: FileFormat = FileFormat {
    id: 112_652_534,
    source_type: SourceType::Wikidata,
    name: "Astound 1.5 Library file format",
    extensions: &["asl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
