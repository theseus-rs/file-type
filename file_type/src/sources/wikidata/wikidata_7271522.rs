use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7271522: FileFormat = FileFormat {
    id: 7_271_522,
    source_type: SourceType::Wikidata,
    name: "Question Object File Format",
    extensions: &["quiz", "quox"],
    media_types: &["application/vnd.quobject-quoxdocument"],
    internal_signatures: &[],
    related_formats: &[],
};
