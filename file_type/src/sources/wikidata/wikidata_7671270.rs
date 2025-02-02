use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7671270: FileFormat = FileFormat {
    id: 7_671_270,
    source_type: SourceType::Wikidata,
    name: "TRANS.TBL",
    extensions: &["TBL"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
