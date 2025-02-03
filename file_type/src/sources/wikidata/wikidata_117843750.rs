use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117843750: FileFormat = FileFormat {
    id: 117_843_750,
    source_type: SourceType::Wikidata,
    name: "IBM IOCA",
    extensions: &["ica"],
    media_types: &["image/x-ioca"],
    internal_signatures: &[],
    related_formats: &[],
};
