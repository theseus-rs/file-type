use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85708317: FileFormat = FileFormat {
    id: 85_708_317,
    source_type: SourceType::Wikidata,
    name: "Calendar Creator File",
    extensions: &["cc3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
