use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_388046: FileFormat = FileFormat {
    id: 388_046,
    source_type: SourceType::Wikidata,
    name: "X-Face",
    extensions: &["face", "xface"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
