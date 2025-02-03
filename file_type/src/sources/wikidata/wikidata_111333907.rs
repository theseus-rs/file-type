use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333907: FileFormat = FileFormat {
    id: 111_333_907,
    source_type: SourceType::Wikidata,
    name: "Roland MT-32 Control + PCM ROM dumps",
    extensions: &["rom"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
