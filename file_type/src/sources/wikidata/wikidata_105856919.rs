use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856919: FileFormat = FileFormat {
    id: 105_856_919,
    source_type: SourceType::Wikidata,
    name: "Google Desktop Gadget manifest",
    extensions: &["gmanifest"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
