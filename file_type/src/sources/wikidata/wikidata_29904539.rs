use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904539: FileFormat = FileFormat {
    id: 29_904_539,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System output file",
    extensions: &["lst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
