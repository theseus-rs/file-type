use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112652459: FileFormat = FileFormat {
    id: 112_652_459,
    source_type: SourceType::Wikidata,
    name: "Astound file format",
    extensions: &["asd"],
    media_types: &["x-application/Astound"],
    internal_signatures: &[],
    related_formats: &[],
};
