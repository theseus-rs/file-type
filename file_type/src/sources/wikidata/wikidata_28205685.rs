use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205685: FileFormat = FileFormat {
    id: 28_205_685,
    source_type: SourceType::Wikidata,
    name: "AMOS Picture Bank",
    extensions: &["abk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
