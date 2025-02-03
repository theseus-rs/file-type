use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67206685: FileFormat = FileFormat {
    id: 67_206_685,
    source_type: SourceType::Wikidata,
    name: "FastCAD DOS file",
    extensions: &["fcd"],
    media_types: &["http://www.wikidata.org/.well-known/genid/ffd72a57f5f0205d2a8bb8294151d36b"],
    internal_signatures: &[],
    related_formats: &[],
};
