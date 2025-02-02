use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473615: FileFormat = FileFormat {
    id: 27_473_615,
    source_type: SourceType::Wikidata,
    name: "Band Interleaved by Line Image File",
    extensions: &["bil"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
