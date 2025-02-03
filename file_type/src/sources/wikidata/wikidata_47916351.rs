use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47916351: FileFormat = FileFormat {
    id: 47_916_351,
    source_type: SourceType::Wikidata,
    name: "Ventura Publisher Vector Graphics",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
