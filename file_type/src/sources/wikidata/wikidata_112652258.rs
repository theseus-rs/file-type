use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112652258: FileFormat = FileFormat {
    id: 112_652_258,
    source_type: SourceType::Wikidata,
    name: "3ds Max Characters",
    extensions: &["chr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
