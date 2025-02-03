use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127812468: FileFormat = FileFormat {
    id: 127_812_468,
    source_type: SourceType::Wikidata,
    name: "Objective-C source code file",
    extensions: &["m"],
    media_types: &["text/x-objective-c"],
    internal_signatures: &[],
    related_formats: &[],
};
