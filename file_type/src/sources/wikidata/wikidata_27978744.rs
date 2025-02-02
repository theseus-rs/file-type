use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27978744: FileFormat = FileFormat {
    id: 27_978_744,
    source_type: SourceType::Wikidata,
    name: "DeluxePaint Animation",
    extensions: &["anm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
