use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116950058: FileFormat = FileFormat {
    id: 116_950_058,
    source_type: SourceType::Wikidata,
    name: "Ulead COOL 360 Project File",
    extensions: &["upj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
