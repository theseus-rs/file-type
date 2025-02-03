use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960128: FileFormat = FileFormat {
    id: 27_960_128,
    source_type: SourceType::Wikidata,
    name: "Computerized Speech Lab NSP",
    extensions: &["nsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
