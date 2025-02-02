use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127327283: FileFormat = FileFormat {
    id: 127_327_283,
    source_type: SourceType::Wikidata,
    name: "Ada file",
    extensions: &["adb"],
    media_types: &["text/x-ada"],
    internal_signatures: &[],
    related_formats: &[],
};
