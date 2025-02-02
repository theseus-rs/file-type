use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76622828: FileFormat = FileFormat {
    id: 76_622_828,
    source_type: SourceType::Wikidata,
    name: "WOLF eBook",
    extensions: &["wol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
