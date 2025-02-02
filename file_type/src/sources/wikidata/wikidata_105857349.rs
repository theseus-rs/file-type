use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857349: FileFormat = FileFormat {
    id: 105_857_349,
    source_type: SourceType::Wikidata,
    name: "Qt Jambi User Interface",
    extensions: &["jui"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
